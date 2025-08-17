![Darstellung der Zelltypen in einem Maushirn](https://raw.githubusercontent.com/plops/slide-tag/main/img/plot-or8.png)


Im Bereich der modernen Genomik ist die Fähigkeit, nicht nur zu verstehen, welche Gene aktiv sind, sondern auch genau
*wo* sie innerhalb eines komplexen Gewebes aktiv sind, eine bedeutende neue Grenze. Dieser Artikel beschreibt ein
leistungsstarkes technologisches Trio, das in Kombination dieses Ziel erreicht: Es misst die Genexpression von Tausenden
einzelner Zellkerne und ordnet sie dann ihrer ursprünglichen Position in einem Gewebe zu. Dieser Ansatz ermöglicht eine
unglaublich detaillierte Ansicht der zellulären Organisation und Funktion biologischer Proben.

Die folgenden Abschnitte führen Sie durch die Schlüsselkomponenten dieses Arbeitsablaufs, von der grundlegenden
Sequenzierungstechnologie bis zu den fortschrittlichen räumlichen Barcoding-Methoden, die diese hochauflösende
Kartierung ermöglichen. Wir werden behandeln:

* **Next-Generation Sequencing (NGS):** Die Kerntechnologie, die das massenhafte Auslesen von DNA-Sequenzen ermöglicht
  und die Grundlage aller modernen Genomanalysen bildet.
* **Einzelkern-RNA-Sequenzierung (snRNA-seq):** Eine Technik, die die Leistungsfähigkeit von NGS nutzt, um die
  Genexpressionsprofile von Tausenden einzelner Kerne zu isolieren und zu analysieren, was die Klassifizierung von
  Zelltypen und -zuständen ermöglicht.
* **Takara Bio Trekker / Slide-tags:** Eine innovative Methode, die die entscheidende „Wo“-Komponente zum „Was“
  hinzufügt. Diese Technologie markiert jeden Zellkern mit einem einzigartigen räumlichen Barcode, während er sich noch
  im Gewebe befindet, und ermöglicht so die rechnerische Rekonstruktion seiner präzisen zweidimensionalen Koordinaten.

Zusammengenommen verwandeln diese Technologien ein biologisches Gewebe in eine hochauflösende Karte, die die zelluläre
Identität, definiert durch die Genexpression, mit ihrer spezifischen Position in einem breiteren anatomischen Kontext
verknüpft.

### Abschnitt 1: Next-Generation Sequencing (NGS)

* **Grundprinzip:** Next-Generation Sequencing (NGS) ist ein Überbegriff für verschiedene Hochdurchsatz-Technologien,
  die die parallele Sequenzierung von Millionen bis Milliarden kurzer DNA-Fragmente gleichzeitig ermöglichen. Die heute
  in der Genomik dominierende Methode, insbesondere für Einzelzell-Anwendungen, ist die **Sequenzierung durch Synthese (
  Sequencing by Synthesis, SBS) von Illumina**.

* **Funktionsweise der Sequenzierung durch Synthese:**
    1. **Bibliothekspräparation:** Der Prozess beginnt mit der Sammlung der zu sequenzierenden DNA (in unserem Fall die
       mit Barcodes versehene cDNA aus dem Einzelnukleus-Workflow). Diese DNA wird fragmentiert, und spezielle
       DNA-Sequenzen, sogenannte **Adapter**, werden an beide Enden jedes Fragments ligiert (angefügt). Diese Adapter
       sind essenziell und dienen als "Andockstellen" für den Sequenzierprozess.
    2. **Cluster-Generierung:** Die vorbereitete DNA-Bibliothek wird auf einen speziellen Glasobjektträger, eine
       sogenannte **Flow Cell**, geladen. Die Oberfläche der Flow Cell ist mit Millionen kurzer DNA-Stränge beschichtet,
       die komplementär zu den Adaptern sind. Jedes DNA-Fragment aus der Bibliothek bindet an einen komplementären
       Strang auf der Flow Cell. Ein Prozess namens **Brückenamplifikation** erzeugt dann einen lokalisierten, dichten
       Cluster aus Tausenden identischer Kopien dieses einen DNA-Fragments. Millionen dieser Cluster, jeder von einem
       einzigen Molekül abstammend, werden gleichzeitig auf der gesamten Flow Cell erzeugt.
    3. **Sequenzierzyklen:** Dies ist der "Synthese"-Teil von SBS. Das Gerät führt eine zyklische chemische Reaktion
       durch, um die Sequenz Base für Base zu lesen.
        * **Schritt A (Einbau):** Das Gerät leitet eine Mischung aller vier DNA-Nukleotide (A, C, G, T) über die Flow
          Cell. Jedes Nukleotid wurde auf zwei Arten modifiziert: Es trägt eine einzigartige fluoreszierende
          Farbmarkierung und besitzt einen "reversiblen Terminator", der verhindert, dass nach ihm weitere Basen
          angefügt werden. In jedem Cluster bindet ein einzelnes Nukleotid an seine komplementäre Base auf dem
          Vorlagenstrang.
        * **Schritt B (Abbildung):** Die Flow Cell wird gewaschen, um alle ungebundenen Nukleotide zu entfernen. Ein
          Laser regt dann den gesamten Objektträger an, und eine hochauflösende Kamera macht ein Bild. Die Farbe der
          Fluoreszenz in jedem Cluster identifiziert, welche Base (A, C, G oder T) gerade hinzugefügt wurde.
        * **Schritt C (Abspaltung):** Eine chemische Reaktion spaltet sowohl die fluoreszierende Markierung als auch den
          reversiblen Terminator ab, sodass im nächsten Zyklus das nächste Nukleotid angefügt werden kann.
    4. **Datenauslese:** Die Schritte A, B und C werden für Hunderte von Zyklen wiederholt. In jedem Zyklus erfasst das
       Gerät die Farbe jedes Clusters auf der Flow Cell. Die endgültige Ausgabe ist eine riesige Textdatei, die
       Millionen von "Reads" enthält – die Sequenzen der DNA-Fragmente (z. B. `GATTACA...`), die jeweils einem
       bestimmten Cluster zugeordnet sind.

### Abschnitt 2: Einzelnukleus-RNA-Sequenzierung (snRNA-seq)

* **Hauptziel:** Die Messung der Menge an messenger-RNA (mRNA) in Tausenden von einzelnen Zellkernen. Dies ermöglicht
  die Klassifizierung von Zelltypen und die Untersuchung zellulärer Zustände basierend auf Genexpressionsprofilen. Es
  ist besonders nützlich für Gewebe, aus denen ganze Zellen schwer zu isolieren sind, wie z. B. Hirngewebe oder
  archivierte gefrorene Proben.

* **Die Rolle kommerzieller Plattformen (10x Genomics, BD Rhapsody):**
    * Plattformen wie das 10x Genomics Chromium oder das BD Rhapsody sind **keine Sequenziergeräte**. Sie sind
      hochentwickelte mikrofluidische "Front-End"-Systeme, die dafür konzipiert sind, die kritischen vorgeschalteten
      Schritte der Einzelzellerfassung und des Barcodings mit hoher Effizienz durchzuführen.
    * Sie automatisieren den Prozess der Isolierung einzelner Zellkerne, deren Verkapselung in Tröpfchen mit
      barkodierten Beads und die Durchführung der molekularen Reaktionen (Lyse, mRNA-Einfang, reverse Transkription) in
      jedem Tröpfchen.
    * Das Endergebnis eines Laufs auf einem 10x Genomics- oder BD Rhapsody-Gerät ist eine "sequenzierfertige
      Bibliothek" – die gepoolte, mit Barcodes versehene cDNA, die dann auf ein NGS-Sequenziergerät (wie ein Illumina
      NovaSeq) geladen wird, um den in Abschnitt 1 beschriebenen eigentlichen Sequenzierschritt durchzuführen.

* **Der snRNA-seq-Mechanismus: Eine schrittweise Aufschlüsselung**
    1. **Isolierung der Zellkerne:**
        * Der Prozess nutzt den strukturellen Unterschied zwischen der Zellmembran (einer einzelnen Lipiddoppelschicht)
          und der robusteren Kernhülle (einer doppelten Membran). Ein mildes Detergens löst selektiv die Zellmembran auf
          und setzt den intakten Zellkern frei.
    2. **Tröpfchen-basierter Einfang (Das "Front-End"):**
        * Ein mikrofluidischer Controller (z. B. 10x Chromium) teilt einen Strom von Zellkernen und einen Strom von
          barkodierten Beads in Tröpfchen von der Größe eines Picoliters auf. Das System ist so kalibriert, dass die
          Verkapselung von einem Zellkern und einem Bead pro Tröpfchen favorisiert wird.
    3. **mRNA-Einfang und molekulares Barcoding:**
        * In jedem Tröpfchen wird der Zellkern lysiert.
        * Der Bead ist mit DNA-Oligonukleotiden beschichtet, die als spezifische Einfangmoleküle fungieren.
        * **Zielmolekül:** Die meisten mRNA-Moleküle in Eukaryoten (wie dem Menschen) haben einen "Poly-A-Schwanz" (eine
          Kette von Adenin-Basen). Dies unterscheidet sie von anderen RNA-Typen (wie ribosomaler RNA (rRNA) oder
          Transfer-RNA (tRNA), die in einer Zelle viel häufiger vorkommen).
        * **Einfanghaken:** Jedes Oligo hat einen "Poly(dT)-Schwanz" (eine Kette von Thymin-Basen), der spezifisch an
          den Poly-A-Schwanz der mRNA bindet.
        * **Barcodes:** Jedes Oligo enthält außerdem zwei entscheidende Identifikatoren:
            * **Zell-Barcode:** Identifiziert den Zellkern in diesem Tröpfchen eindeutig.
            * **Eindeutiger Molekularer Identifikator (Unique Molecular Identifier, UMI):** Identifiziert jedes einzelne
              eingefangene mRNA-Molekül eindeutig.
    4. **Bibliothekspräparation und Sequenzierung:**
        * Nach den Reaktionen in den Tröpfchen wird die Emulsion gebrochen und die nun mit Barcodes versehene cDNA wird
          gepoolt. Adapter werden zu dieser cDNA hinzugefügt, um sie mit einem NGS-Gerät kompatibel zu machen. Diese
          endgültige Sammlung von Molekülen ist die "Bibliothek".
        * Diese Bibliothek wird auf ein NGS-Sequenziergerät geladen, das die Sequenz des mRNA-Fragments sowie den
          angehängten Zell-Barcode und das UMI liest.

* **Schlüssel-Leistungskennzahlen (Key Performance Indicators, KPIs) für die Datenqualität**
    * **Anzahl der erfassten Zellkerne:** Die Gesamtzahl der Zellen mit ausreichenden Daten für die Analyse.
    * **Median der Gene pro Zellkern:** Der Median der Anzahl verschiedener Gene, die pro Zellkern nachgewiesen wurden;
      ein Maß für die Sensitivität.
    * **Median der UMIs pro Zellkern:** Der Median der Anzahl einzigartiger mRNA-Moleküle, die pro Zellkern nachgewiesen
      wurden; ein Maß für die Einfangeffizienz.
    * **Doublet-Rate:** Der geschätzte Prozentsatz von "Barcodes", die tatsächlich zwei oder mehr Zellkerne
      repräsentieren, die im selben Tröpfchen eingeschlossen wurden.
    * **Anteil mitochondrialer RNA:** Bei snRNA-seq sollte dieser Prozentsatz sehr niedrig sein. Ein hoher Wert kann auf
      eine Kontamination aus dem Zytoplasma aufgrund von aufgebrochenen Zellkernen hinweisen.

| **KPI**                            | **Was wird gemessen?**                                                                                                                                                 | **Wie wird es berechnet?**                                                                                                                                                                                            | **Typischer Bereich für hohe Qualität**                                               | **Beeinflussende Faktoren**                                                                                                                                                        |
|:-----------------------------------|:-----------------------------------------------------------------------------------------------------------------------------------------------------------------------|:----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|:--------------------------------------------------------------------------------------|:-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **Anzahl der erfassten Zellkerne** | Die Gesamtzahl der einzelnen Zellkerne, für die Daten gewonnen werden.                                                                                                 | Die Anzahl der einzigartigen Zell-Barcodes mit einer ausreichenden Anzahl von Reads.                                                                                                                                  | Tausende bis Zehntausende, abhängig von der Plattform und den experimentellen Zielen. | Zellkonzentration, Leistung des mikrofluidischen Geräts und nachgeschaltete Filterkriterien.                                                                                       |
| **Median der Gene pro Zellkern**   | Der Median der Anzahl verschiedener Gene, die in einem einzelnen Zellkern nachgewiesen werden. Dies spiegelt die Sensitivität des Assays wider.                        | Für jeden Zellkern (Zell-Barcode) wird die Anzahl der Gene mit mindestens einem Read gezählt. Dann wird der Median dieser Zählungen über alle Zellkerne ermittelt.                                                    | 500 - 5.000+ (stark abhängig vom Zelltyp und der Sequenziertiefe).                    | RNA-Qualität, Effizienz der reversen Transkription und Sequenziertiefe.                                                                                                            |
| **Median der UMIs pro Zellkern**   | Der Median der Anzahl einzigartiger mRNA-Moleküle, die pro Zellkern nachgewiesen werden. Dies ist ein Maß für die Komplexität der Bibliothek und die Einfangeffizienz. | Für jeden Zellkern wird die Anzahl der einzigartigen UMIs gezählt. Dann wird der Median dieser Zählungen über alle Zellkerne ermittelt.                                                                               | 1.000 - 20.000+ (stark abhängig vom Zelltyp und der Sequenziertiefe).                 | RNA-Gehalt des Zellkerns, Einfangeffizienz der Beads und Zyklen der PCR-Amplifikation.                                                                                             |
| **Anteil der Reads in Zellkernen** | Der Prozentsatz der Sequenzier-Reads, die mit gültigen Zell-Barcodes assoziiert sind.                                                                                  | (Gesamtzahl der Reads mit gültigen Zell-Barcodes / Gesamtzahl der Sequenzier-Reads) * 100 %.                                                                                                                          | > 50 %                                                                                | Qualität der Einzelkernsuspension, Effizienz der Tröpfchen-Verkapselung und Genauigkeit der Barcode-Identifizierung.                                                               |
| **Sequenziersättigung**            | Das Ausmaß, in dem die Sequenziertiefe die Komplexität der Bibliothek erfasst hat.                                                                                     | 1 - (Anzahl der einzigartigen UMIs / Gesamtzahl der Reads für einen gegebenen Zell-Barcode). Ein höherer Wert deutet darauf hin, dass weiteres Sequenzieren wahrscheinlich nicht viele neue Moleküle nachweisen wird. | > 30-50 %, abhängig vom gewünschten Grad der Gen-Detektion.                           | Gesamtzahl der Sequenzier-Reads im Verhältnis zur Komplexität der cDNA-Bibliothek.                                                                                                 |
| **Anteil mitochondrialer RNA**     | Der Prozentsatz der Reads, die dem mitochondrialen Genom zugeordnet werden können.                                                                                     | (Anzahl der Reads, die auf mitochondriale Gene abgebildet werden / Gesamtzahl der Reads für einen Zellkern) * 100 %.                                                                                                  | Typischerweise < 5 % für snRNA-seq.                                                   | Bei snRNA-seq deutet ein hoher Anteil an mitochondrialer RNA oft auf eine zytoplasmatische Kontamination durch unvollständige Zelllyse oder eine Beschädigung der Kernmembran hin. |
| **Doublet-Rate**                   | Der Prozentsatz der "Zellen", die tatsächlich zwei oder mehr Zellkerne sind, die im selben Tröpfchen verkapselt wurden.                                                | Schätzung erfolgt computergestützt auf Basis von Genexpressionsprofilen oder experimentell durch Mischen von Proben.                                                                                                  | < 1 % pro 1.000 geladene Zellen.                                                      | Die Konzentration der Zellkerne, die in das mikrofluidische Gerät geladen werden.                                                                                                  |

### Abschnitt 3: Takara Bio Trekker / Slide-tags: Hinzufügen räumlicher Koordinaten zu Einzelnukleus-Daten

* **Grundprinzip:** Diese Methode markiert einzelne Zellkerne mit räumlichen Barcodes, *während sie sich noch in einem
  intakten Gewebeschnitt befinden*. Anstatt zelluläre Moleküle auf einer Oberfläche einzufangen, werden Barcodes von
  einer Oberfläche in das Gewebe freigesetzt, was hochwertige Einzelzelldaten ermöglicht, während die ursprünglichen
  räumlichen Koordinaten erhalten bleiben.

* **Der Mechanismus**
    1. **Das barkodierte Array:** Die Technologie verwendet einen Glasobjektträger, der mit einer dichten, zufälligen
       Monoschicht von 10-Mikrometer-DNA-barkodierten Beads beschichtet ist. Die enorme Vielfalt einzigartiger *
       *räumlicher Barcodes** wird mittels kombinatorischer Split-Pool-Synthese erzeugt. Die physische (x, y)-Koordinate
       jeder einzigartigen Barcode-Sequenz auf diesem Array wird vorab durch eine *in-situ*-Sequenzierreaktion direkt
       auf dem Objektträger bestimmt, wodurch eine definitive digitale Karte erstellt wird.
    2. **Räumliches Tagging im Gewebe:** Ein frisch gefrorener Gewebeschnitt (typischerweise 20 µm dick) wird auf den
       Objektträger gelegt. UV-Licht spaltet Linker auf den Beads, wodurch die räumlichen Barcodes freigesetzt werden,
       um in das Gewebe zu diffundieren und die Zellkerne zu markieren. Dieser vorgeschaltete Markierungsschritt
       verlängert den Arbeitsablauf nur um 10-60 Minuten.
    3. **Erzeugung zweier Bibliotheken in Tröpfchen:** Nach der Markierung wird das Gewebe in eine Einzelkernsuspension
       aufgelöst und mit einer tröpfchenbasierten Plattform (z. B. 10x Genomics) verarbeitet.
        * **Verkapselung:** Ein einzelner, räumlich markierter Zellkern wird in einem Tröpfchen mit einem einzelnen 10x
          Genomics Gel-Bead verkapselt. Dieser Bead setzt Tausende seiner eigenen Oligonukleotide frei, die jeweils
          einen **Zell-Barcode (Cell Barcode, CB)** – der als eindeutige Adressmarkierung für diesen spezifischen
          Zellkern dient – einen Unique Molecular Identifier (UMI) und Primer enthalten.
        * **Parallele cDNA-Synthese:** Innerhalb des Tröpfchens wird der Zellkern lysiert. Die reverse Transkription
          erzeugt dann parallel zwei verschiedene Arten von cDNA-Molekülen, die beide nun mit demselben **Zell-Barcode**
          verknüpft sind:
            * **Genexpressions (GEX)-cDNA:** Die mRNA des Zellkerns wird eingefangen, wodurch ein langes cDNA-Molekül
              mit der Struktur **[Gensequenz] + [UMI] + [Zell-Barcode]** entsteht.
            * **Räumlicher Barcode (SB)-cDNA:** Die räumlichen Barcodes von Trekker werden eingefangen, wodurch ein
              separates und viel kürzeres cDNA-Molekül mit der Struktur **[Räumlicher Barcode] + [UMI] + [Zell-Barcode]**
              entsteht.
        * **Physische Trennung:** Nachdem die Tröpfchen aufgebrochen wurden, werden die beiden cDNA-Typen aus der
          gepoolten Lösung physisch getrennt, typischerweise aufgrund ihres signifikanten Größenunterschieds. Sie werden
          dann zu zwei unterschiedlichen Sequenzierbibliotheken amplifiziert: einer GEX-Bibliothek und einer
          SB-Bibliothek.
    4. **Computergestützte Positionsrekonstruktion:** Durch die Analyse der Sequenzierdaten aus der SB-Bibliothek wird
       das spezifische Verhältnis verschiedener räumlicher Barcodes, die einem einzelnen Zell-Barcode zugeordnet sind,
       bestimmt. Dieses Verhältnis ermöglicht im Abgleich mit den vorab kartierten Koordinaten der Beads die hochpräzise
       Berechnung der ursprünglichen (x, y)-Position des Zellkerns.

* **Leistung und Sequenziereffizienz**
    * **Räumliche Präzision:** Die Triangulationsmethode erreicht eine hohe räumliche Lokalisierungsgenauigkeit, die auf
      **~3,5 µm** geschätzt wird.
    * **Sequenzierstrategie:** Die beiden Bibliotheken werden mit unterschiedlichen "Tiefen" (Gesamtzahl der Reads)
      sequenziert, da sie Probleme unterschiedlicher Komplexität lösen. Ein "Read" ist eine kurze DNA-Sequenz von 50-150
      Basenpaaren, die von der Sequenziermaschine erzeugt wird.
        * **Genexpressionsbibliothek (Hohe Tiefe):** Benötigt **20.000-50.000 Reads pro Zellkern**. Diese hohe Tiefe ist
          notwendig, um die Tausenden verschiedener mRNAs in einer Zelle umfassend abzutasten, von denen viele sehr
          selten sind und eine große Anzahl von Reads erfordern, um zuverlässig nachgewiesen und gezählt zu werden.
        * **Räumliche Barcode-Bibliothek (Geringe Tiefe):** Benötigt nur **1.000-5.000 Reads pro Zellkern**. Der Grund
          dafür ist, dass das Ziel viel einfacher ist: die Handvoll bekannter räumlicher Barcodes zu identifizieren, die
          ein Zellkern aufgenommen hat. Eine kleine Anzahl von Reads ist ausreichend, um diese kurzen Barcode-Sequenzen
          sicher zu erkennen und ihre relativen Verhältnisse zu bestimmen.

## Referenzen

1. [YouTube-Video mit einigen Visualisierungen von Slide-tag-Messungen unter Verwendung eines kommerziellen Kits](https://youtu.be/rd2G3yjWszQ?t=385)
2. [Slide-tag-Paper (Nature)](https://www.nature.com/articles/s41586-023-06837-4)
3. [Animation, die die reverse Transkriptase zeigt](https://www.youtube.com/watch?v=SURGNo44wmU)

[ Dieser Text wurde mit Hilfe von KI erstellt ]