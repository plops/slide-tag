#!/bin/bash

# This script collects the Rust codebase design documents in 
# order to help changing the design before finalizing the implementation.


ROOT_DIR="/home/kiel/stage/slide-tag/companies/roche/rs_scrape_jobs/"
SCRAPE_JOBS_DIR="${ROOT_DIR}/scrape_jobs"

{
declare -a FILES=(
    "plans/03_tasks.md"
    "Cargo.toml"
    "src/"*.rs
    "src/bin/"*.rs
)

for i in "${FILES[@]}"; do
    if [ -f "$i" ]; then
        SIZE_KB=$(du -k "$i" | cut -f1)
        echo "LOG: Processing $i (${SIZE_KB} KB)" >&2
        echo "// start of $i"
        cat "$i"
        echo "// end of $i"
    else
        echo "ERROR: File not found: $i" >&2
        exit 1
    fi
done

cat << 'EOF'

--- PROMPT ---

Ich habe gerade Stufe 6 des bisherigen Plans 03_tasks.md implementiert.

Dabei ist sind mir Aenderungen aufgefallen, die wir am besten jetzt in den Plan einbauen.
Die Aenderungen betreffen nur Stufen nach Stufe 6, wir koennen also die restlichen Stufen
unveraendert lassen und Stufe 7 und hoeher komplett neu formulieren.

Erstens:
Im free tier von Gemini habe ich nur wenige Requests pro Tag. In einem Request koennen jedoch mehrere
Jobbeschreibungen verarbeitet werden. Wie viele, haengt mit der Laenge der Inputs zusammen. Typischerweise
bleiben die strukturierten JSON Antworten valide solange der Input um die 20000 Token bleibt.

Gibt es eine Rust bibliothek, die die gemini token im eingangstext zaehlt? Falls nicht dann wuerde
ich die Annahme machen, dass ein token = 0.8 worte. und dann wuerde ich das limit halt auf 18000 worte setzen oder so.

wesentlich ist jedoch dass wir beim konstruieren der anfrage die worte zaehlen muessen und aufhoeren, bevor wir das limit
ueberschreiten. wir koennten z.b. fuer jeden job ein subprompt erstellen, die worte in jedem subprompt zaehlen
und dann die subprompts zu einer groessenbegrenzten anfrage zusammenfuegen mit einer gesamtzahl von worten die
unter dem limit liegt.

zweitens:
Der bisherig plan beruecksichtigt bisher nicht, dass wir mehrere kandidatenprofile haben und dass wir fuer jeden kandidaten
eine separate anfrage an die kI senden muessen. ausserdem muessen die resultate pro kandidat in der datenbank abgespeichert
werden. zum debugging moechte ich auch dass es moeglich ist, eine job profil fuer einen kandidaten mehrmals zusammenzufassen
und in der datenbank zu speichern (z.b. um ein anderes modell zu testen oder wenn der kandidat sein profil aendert).
wenn mehrere job auswertungen fuer einen kandidaten existieren, dann sollte die neueste auswertung verwendet werden.


drittens:
bisher ist nur geplant eine pdf mit den passenden jobs fuer jeden kandidaten zu erstellen.

ich finde die software waere viel besser, wenn sie eine webseite praesentiert, wo sich ein kandidat anmelden kann
(mit google oauth, oder github oauth) und dort seine eigenen profile hochladen kann. dann koennte er die profile
mit den jobs vergleichen und die besten jobs auswaehlen. ich moechte dass der implementierungsplan diesbezueglich
ueberarbeitet wird, so dass wir schrittweise die neue funktion implementieren koennen. und zwischendurch immer testen.

fuer den betrieb der webseite ist es auch erforderlich, dass das scraping automatisch laeuft (also z.b. jede nacht) und dass
die neuen jobs oder jobs bei denen sich die daten geandert haben in der datenbank gespeichert werden.
da es sich nur um 150 jobs pro nacht handelt und jede job beschreibung vielleicht 2 a4 seiten umfasst, gibt es soweit
ich sehe kein problem, die historische entwicklung der von roche geposteten jobs alle in der datenbank zu speichern.

das waere vielleicht auch interessant, zu sehen wie sich so ein posting aendert.

fuer den automatischen ablauf finde ich es wichtig, dass die daten alle in der datenbank bleiben und z.b. nicht irgendwelche html seiten in jobs_html gespeichert werden.
die html dateien will ich aber glaube ich nicht in der datenbank halten.
auch die json daten aus dem html sind recht umfassend. ich gehe einfach davon aus, dass unser json parser schon alle 
wichtigen informationen extrahiert und in der datenbank speichert. also werden wir auch nicht das rohe json speichern

vielleicht sollten wir debug optionen einrichten, um die json und/oder html daten zu speichern (mit identifier in einem ordner mit datetime, ich denke das sollte genuegen).



EOF
} | xclip -selection clipboard

echo "Codebase and updated Plan updating prompt have been copied to the clipboard."
