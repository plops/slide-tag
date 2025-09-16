# python
import asyncio
import aiohttp
import time
import os
import re
from urllib.parse import urlparse, unquote
from bs4 import BeautifulSoup

JOBS_FILE = "jobs.txt"
OUT_HTML = "jobs_html"
OUT_TEXT = "jobs_text"
CONCURRENCY = 8
TIMEOUT = 15
RETRIES = 3
USER_AGENT = "Mozilla/5.0 (compatible; JobScraper/1.0; +https://example.org/bot)"


def sanitize_name(url: str, index: int) -> str:
    p = urlparse(url)
    name = os.path.basename(unquote(p.path)) or f"job-{index}"
    name = re.sub(r"[^A-Za-z0-9._-]", "-", name)
    name = name[:150]
    return f"{index:04d}-{name}"


def extract_text_from_html(html: str) -> str:
    soup = BeautifulSoup(html, "html.parser")
    # prefer article/main, else fallback to largest text block
    candidates = soup.find_all(["article", "main"])
    if not candidates:
        candidates = soup.find_all("div")
    best = max(candidates, key=lambda t: len(t.get_text(strip=True)), default=soup)
    text = best.get_text(separator="\n", strip=True)
    # normalize whitespace
    text = re.sub(r"\n{2,}", "\n\n", text)
    return text


async def fetch(
    session: aiohttp.ClientSession, url: str, retries: int = RETRIES
) -> str | None:
    for attempt in range(1, retries + 1):
        try:
            async with session.get(url, timeout=TIMEOUT) as resp:
                if resp.status == 200:
                    return await resp.text()
                else:
                    # non-200 -> treat as failure
                    print(f"Warning: {url} returned status {resp.status}")
                    return None
        except (aiohttp.ClientError, asyncio.TimeoutError) as e:
            wait = 1.5**attempt
            print(
                f"Fetch error ({attempt}/{retries}) for {url}: {e}. Retrying in {wait:.1f}s"
            )
            await asyncio.sleep(wait)
    print(f"Failed to fetch {url} after {retries} attempts")
    return None


async def worker(
    sem: asyncio.Semaphore, session: aiohttp.ClientSession, url: str, idx: int
):
    async with sem:
        name = sanitize_name(url, idx)
        html_path = os.path.join(OUT_HTML, name + ".html")
        text_path = os.path.join(OUT_TEXT, name + ".txt")

        if os.path.exists(html_path) and os.path.exists(text_path):
            print(f"Skipping (exists): {url}")
            return

        html = await fetch(session, url)
        if not html:
            return

        # write html
        with open(html_path, "w", encoding="utf-8") as f:
            f.write(html)

        # extract and write text
        try:
            text = extract_text_from_html(html)
        except Exception:
            text = re.sub(r"<[^>]+>", "", html)
        with open(text_path, "w", encoding="utf-8") as f:
            f.write(text)

        print(f"Saved: {url} -> {name}")


async def main():
    if not os.path.exists(JOBS_FILE):
        print(f"File not found: {JOBS_FILE}")
        return

    os.makedirs(OUT_HTML, exist_ok=True)
    os.makedirs(OUT_TEXT, exist_ok=True)

    with open(JOBS_FILE, "r", encoding="utf-8") as f:
        urls = [line.strip() for line in f if line.strip()]

    sem = asyncio.Semaphore(CONCURRENCY)
    timeout = aiohttp.ClientTimeout(total=None)
    headers = {"User-Agent": USER_AGENT, "Accept": "text/html,application/xhtml+xml"}

    async with aiohttp.ClientSession(timeout=timeout, headers=headers) as session:
        tasks = [worker(sem, session, url, i + 1) for i, url in enumerate(urls)]
        start = time.time()
        await asyncio.gather(*tasks)
        print(f"Done. Downloaded {len(urls)} items in {time.time() - start:.1f}s")


if __name__ == "__main__":
    asyncio.run(main())
