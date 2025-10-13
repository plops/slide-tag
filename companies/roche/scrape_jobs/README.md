|    | in               | out                        | comment                                                        |
| 01 | browser          | jobs.txt                   | links to open job descriptions                                 |
| 02 | jobs.txt         | jobs_{html,text}           | full-text descriptios for open jobs                            |
| 03 | jobs_html/*.html | jobs_html/*.json           | extract json database tdump from html files and save it        |
| 04 | jobs_html/*.html | jobs_minutils.db           | convert all json objects into sqlite                           |
| 05 | jobs_minutils.db | df_with_ai_annotations.csv | create bullet list summaries of the full text job descriptions |
|    |                  |                            |                                                                |
