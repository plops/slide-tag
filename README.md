![Rendering of cell classes of mouse brain](https://raw.githubusercontent.com/plops/slide-tag/main/img/plot-or8.png)

In the realm of modern genomics, the ability to understand not just what genes are active but precisely *where* they are
active within a complex tissue is a significant frontier. This article breaks down a powerful technological trio that,
when combined, achieves this goal: it measures gene expression from thousands of individual cell nuclei and then maps
them back to their original location in a tissue. This approach provides an incredibly detailed view of the cellular
organization and function of biological samples.

The following sections will guide you through the key components of this workflow, from the fundamental sequencing
technology to the advanced spatial barcoding methods that make this high-resolution mapping possible. We will cover:

* **Next-Generation Sequencing (NGS):** The core engine that enables the large-scale reading of DNA sequences, which is
  the foundation for all modern genomic analyses.
* **Single-Nucleus RNA Sequencing (snRNA-seq):** A technique that applies the power of NGS to isolate and analyze the
  gene expression profiles of thousands of individual nuclei, allowing for the classification of cell types and states.
* **Takara Bio Trekker / Slide-tags:** An innovative method that adds the crucial "where" component to the "what." This
  technology tags each nucleus with a unique spatial barcode while it is still in the tissue, allowing for the
  computational reconstruction of its precise two-dimensional coordinates.

Together, these technologies transform a biological tissue into a high-resolution map, linking cellular identity, as
defined by gene expression, to its specific location within a broader anatomical context.

### Section 1: Next-Generation Sequencing (NGS)

* **Core Principle:** Next-Generation Sequencing (NGS) is a term for several high-throughput technologies that allow for
  the parallel sequencing of millions to billions of short DNA fragments simultaneously. The dominant method used in
  genomics today, particularly for single-cell applications, is **Illumina's Sequencing by Synthesis (SBS)**.

* **How Sequencing by Synthesis Works:**
    1. **Library Preparation:** The process starts with the collection of DNA to be sequenced (in our case, the barcoded
       cDNA from the single-nucleus workflow). This DNA is fragmented, and special DNA sequences called **adapters** are
       ligated to both ends of each fragment. These adapters are essential, acting as "docking sites" for the sequencing
       process.
    2. **Cluster Generation:** The prepared DNA library is loaded onto a specialized glass slide called a **flow cell**.
       The surface of the flow cell is coated with millions of short DNA strands that are complementary to the adapters.
       Each DNA fragment from the library binds to a complementary strand on the flow cell. A process called **bridge
       amplification** then creates a localized, dense cluster of thousands of identical copies of that single DNA
       fragment. Millions of these clusters are generated simultaneously across the flow cell, each originating from a
       single molecule.
    3. **Sequencing Cycles:** This is the "synthesis" part of SBS. The machine performs a cyclical chemical reaction to
       read the sequence one base at a time.
        * **Step A (Incorporate):** The machine flows a mixture of all four DNA nucleotides (A, C, G, T) across the flow
          cell. Each nucleotide has been modified in two ways: it carries a unique fluorescent color tag, and it has a "
          reversible terminator" that prevents any more bases from being added after it. A single nucleotide binds to
          its complementary base on the template strand in each cluster.
        * **Step B (Image):** The flow cell is washed to remove any unbound nucleotides. A laser then excites the entire
          slide, and a high-resolution camera takes a picture. The color of the fluorescence in each cluster identifies
          which base (A, C, G, or T) was just added.
        * **Step C (Cleave):** A chemical reaction cleaves off both the fluorescent tag and the reversible terminator,
          allowing the next nucleotide to be added in the subsequent cycle.
    4. **Data Readout:** Steps A, B, and C are repeated for hundreds of cycles. In each cycle, the machine records the
       color of every cluster on the flow cell. The final output is a massive text file containing millions of "reads"
       —the sequences of the DNA fragments (e.g., `GATTACA...`), each tied to a specific cluster.

### Section 2: Single-Nucleus RNA Sequencing (snRNA-seq)

* **Primary Goal:** To measure the abundance of messenger RNA (mRNA) in thousands of individual nuclei. This allows for
  the classification of cell types and the study of cellular states based on gene expression profiles. It is especially
  useful for tissues where whole cells are difficult to isolate, such as brain tissue or archived frozen samples.

* **The Role of Commercial Platforms (10x Genomics, BD Rhapsody):**
    * Platforms like the 10x Genomics Chromium or the BD Rhapsody are **not sequencers**. They are sophisticated
      microfluidic "front-end" systems designed to perform the critical upstream steps of single-cell capture and
      barcoding with high efficiency.
    * They automate the process of isolating single nuclei, encapsulating them in droplets with barcoded beads, and
      performing the molecular reactions (lysis, mRNA capture, reverse transcription) inside each droplet.
    * The final output of a 10x Genomics or BD Rhapsody instrument run is a "sequencing-ready library"—the pooled,
      barcoded cDNA that is then loaded onto an NGS sequencer (like an Illumina NovaSeq) for the actual sequencing step
      described in Section 1.

* **The snRNA-seq Mechanism: A Step-by-Step Breakdown**
    1. **Nucleus Isolation:**
        * The process exploits the structural difference between the cell membrane (a single lipid bilayer) and the more
          robust nuclear envelope (a double membrane). A mild detergent selectively dissolves the cell membrane,
          releasing the intact nucleus.
    2. **Droplet-based Capture (The "Front-End"):**
        * A microfluidic controller (e.g., 10x Chromium) partitions a stream of nuclei and a stream of barcoded beads
          into picoliter-sized droplets. The system is calibrated to favor the encapsulation of one nucleus and one bead
          per droplet.
    3. **mRNA Capture and Molecular Barcoding:**
        * Inside each droplet, the nucleus is lysed.
        * The bead is coated with DNA oligonucleotides that act as specific capture molecules.
        * **Capture Target:** Most mRNA molecules in eukaryotes (like humans) have a "poly-A tail" (a string of Adenine
          bases). This distinguishes it from other types of RNA (like ribosomal RNA (rRNA) or transfer RNA (tRNA), which
          are much more abundant in a cell).
        * **Capture Hook:** Each oligo has a "poly(dT) tail" (a string of Thymine bases) that specifically binds to the
          mRNA's poly-A tail.
        * **Barcodes:** Each oligo also contains two crucial identifiers:
            * **Cell Barcode:** Uniquely identifies the nucleus in that droplet.
            * **Unique Molecular Identifier (UMI):** Uniquely identifies each individual mRNA molecule captured.
    4. **Library Preparation and Sequencing:**
        * After the reactions in the droplets [3], the emulsion is broken, and the now-barcoded cDNA is pooled. Adapters
          are
          added to this cDNA to make it compatible with an NGS machine. This final collection of molecules is the "
          library."
        * This library is loaded onto an NGS sequencer, which reads the sequence of the mRNA fragment as well as the
          attached cell barcode and UMI.

* **Key Performance Indicators (KPIs) for Data Quality**
    * **Number of Nuclei Captured:** The total number of cells with sufficient data for analysis.
    * **Median Genes per Nucleus:** The median number of different genes detected per nucleus; a measure of sensitivity.
    * **Median UMIs per Nucleus:** The median count of unique mRNA molecules detected per nucleus; a measure of capture
      efficiency.
    * **Doublet Rate:** The estimated percentage of "barcodes" that actually represent two or more nuclei captured in
      the same droplet.
    * **Mitochondrial RNA Content:** In snRNA-seq, this percentage should be very low. A high value can indicate
      contamination from the cytoplasm due to ruptured nuclei.
    * **Total Nuclei Recovery Rate:** The percentage of nuclei from the original tissue slice that are successfully
      sequenced and spatially mapped is currently around 25% but could be improved by modifying protocol for specific
      tissues.

| **KPI**                         | **What it Measures**                                                                                                                 | **How it's Calculated**                                                                                                                                        | **Typical High-Quality Range**                                                    | **Factors Influencing the Metric**                                                                                                                    |
|:--------------------------------|:-------------------------------------------------------------------------------------------------------------------------------------|:---------------------------------------------------------------------------------------------------------------------------------------------------------------|:----------------------------------------------------------------------------------|:------------------------------------------------------------------------------------------------------------------------------------------------------|
| **Number of Nuclei Captured**   | The total number of individual nuclei for which data is obtained.                                                                    | The number of unique cell barcodes with a sufficient number of reads.                                                                                          | Thousands to tens of thousands, depending on the platform and experimental goals. | Cell concentration, microfluidic device performance, and downstream filtering criteria.                                                               |
| **Median Genes per Nucleus**    | The median number of distinct genes detected in a single nucleus. This reflects the sensitivity of the assay.                        | For each nucleus (cell barcode), count the number of genes with at least one read. Then, find the median of these counts across all nuclei.                    | 500 - 5,000+ (highly dependent on cell type and sequencing depth).                | RNA quality, reverse transcription efficiency, and sequencing depth.                                                                                  |
| **Median UMIs per Nucleus**     | The median number of unique mRNA molecules detected per nucleus. This is a measure of the library complexity and capture efficiency. | For each nucleus, count the number of unique UMIs. Then, find the median of these counts across all nuclei.                                                    | 1,000 - 20,000+ (highly dependent on cell type and sequencing depth).             | RNA content of the nucleus, bead capture efficiency, and PCR amplification cycles.                                                                    |
| **Fraction of Reads in Nuclei** | The percentage of sequencing reads that are associated with valid cell barcodes.                                                     | (Total reads with valid cell barcodes / Total sequencing reads) * 100%.                                                                                        | > 50%                                                                             | Quality of the single-nucleus suspension, efficiency of droplet encapsulation, and accuracy of barcode identification.                                |
| **Sequencing Saturation**       | The extent to which the sequencing depth has captured the complexity of the library.                                                 | 1 - (Number of unique UMIs / Total reads for a given cell barcode). A higher value indicates that further sequencing is unlikely to detect many new molecules. | > 30-50%, depending on the desired level of gene detection.                       | Total number of sequencing reads relative to the complexity of the cDNA library.                                                                      |
| **Mitochondrial RNA Content**   | The percentage of reads that map to the mitochondrial genome.                                                                        | (Number of reads mapping to mitochondrial genes / Total reads for a nucleus) * 100%.                                                                           | Typically < 5% for snRNA-seq.                                                     | In snRNA-seq, high mitochondrial RNA is often indicative of cytoplasmic contamination due to incomplete cell lysis or damage to the nuclear membrane. |
| **Doublet Rate**                | The percentage of "cells" that are actually two or more nuclei encapsulated in the same droplet.                                     | Estimated computationally based on gene expression profiles or experimentally through sample mixing.                                                           | < 1% per 1,000 cells loaded.                                                      | The concentration of nuclei loaded into the microfluidic device.                                                                                      |
| **Total Nuclei Recovery Rate**  | The percentage of nuclei from the original tissue slice that are successfully sequenced and spatially mapped.                        | Estimated by comparing the number of mapped nuclei to the expected number of cells in the tissue area.                                                         | ~10% (v1 protocol), now improved to 25-30% with v2.                               | Efficiency of tissue dissociation, nuclear isolation, and spatial barcode capture.                                                                    |

### Section 3: Takara Bio Trekker / Slide-tags: Adding Spatial Coordinates to Single-Nucleus Data

* **Core Principle:** This method tags individual cell nuclei with spatial barcodes *while they are still in an intact
  tissue slice*. Instead of capturing molecules onto a surface, it releases barcodes from a surface up into the tissue,
  allowing for high-quality single-cell data while retaining the original spatial coordinates.

* **The Mechanism**
    1. **The Barcoded Array:** The technology uses a glass slide coated with a dense, random monolayer of 10-micron
       DNA-barcoded beads. The massive diversity of unique **spatial barcodes** is generated using split-pool
       combinatorial synthesis. The physical (x, y) coordinate of every unique barcode sequence on this array is
       can be determined beforehand by performing an *in situ* sequencing reaction directly on the slide, creating a
       definitive
       digital map. An alternative imaging-free method allows for its computational reconstruction. Using the sequence
       of the local
       barcodes captured by many individual cells, a matrix of pairwise distances between all barcodes is generated,
       which is then used to calculate the absolute (x, y) coordinates of every bead with remarkable accuracy and
       distances of multiple centimeters (see [4] at 1:05:03).
    2. **Spatial Tagging in Tissue:** A fresh-frozen tissue slice (typically 20 µm thick) is placed on the slide. UV
       light cleaves linkers on the beads, releasing the spatial barcodes to diffuse into the tissue and tag the nuclei.
       The attachment is non-specifical but robust, likely through charge interactions with histone proteins.
       The upstream tagging step adds only 10-60 minutes to the workflow.
    3. **Dual Library Generation in Droplets:** After tagging, the tissue is dissociated into a single-nucleus
       suspension and processed using a droplet-based platform (e.g., 10x Genomics).
        * **Encapsulation:** A single, spatially-tagged nucleus is encapsulated in a droplet with a single 10x Genomics
          gel bead. This bead releases thousands of its own oligonucleotides, each containing a **Cell Barcode (CB)**
          —which acts as a unique address label for that specific nucleus—a Unique Molecular Identifier (UMI), and
          primers.
        * **Parallel cDNA Synthesis:** Inside the droplet, the nucleus is lysed. Reverse transcription then creates two
          different types of cDNA molecules in parallel, both of which are now linked to the same **Cell Barcode**:
            * **Gene Expression (GEX) cDNA:** The nucleus's mRNA is captured, creating a long cDNA molecule structured
              as: **[Gene Sequence] + [UMI] + [Cell Barcode]**.
            * **Spatial Barcode (SB) cDNA:** The Trekker spatial barcodes are captured, creating a separate and much
              shorter cDNA molecule structured as: **[Spatial Barcode] + [UMI] + [Cell Barcode]**.
        * **Physical Separation:** After the droplets are broken, the two types of cDNA are physically separated from
          the pooled solution, typically based on their significant size difference. They are then amplified into two
          distinct sequencing libraries: a GEX library and an SB library.
    4. **Computational Position Reconstruction:** By analyzing the sequencing data from the SB library, the specific
       ratio of different spatial barcodes associated with a single Cell Barcode is determined. This ratio, when
       compared against the pre-mapped coordinates of the beads, allows the original (x, y) position of the nucleus to
       be calculated with high precision.

* **Performance and Sequencing Efficiency**
    * **Spatial Precision:** The triangulation method achieves a high spatial localization accuracy, estimated to be ~
      3.5 µm.
    * **Sequencing Strategy:** The two libraries are sequenced to different "depths" (total number of reads) because
      they solve problems of different complexity. A "read" is a short DNA sequence of 50-150 base pairs generated by
      the sequencing machine.
        * **Gene Expression Library (High Depth):** Requires **20,000-50,000 reads per nucleus**. This high depth is
          necessary to comprehensively sample the thousands of different mRNAs in a cell, many of which are very rare
          and require a large number of reads to be reliably detected and counted.
        * **Spatial Barcode Library (Low Depth):** Requires only **1,000-5,000 reads per nucleus**. This is because the
          goal is much simpler: to identify the handful of known spatial barcodes a nucleus has absorbed. A small number
          of reads is sufficient to confidently recognize these short barcode sequences and determine their relative
          ratios.

# Continue reading

Have a look at [DATASETS.md](https://github.com/plops/slide-tag/blob/main/DATASETS.md) the discussion of the input and
output data and the raw processing required for the slide-tag method.

I started investigating companies related to this technology. Have a look at 
[Roche](https://github.com/plops/slide-tag/blob/main/companies/roche/reports/2024.md).

## References

1. [YouTube video with some visualizations of slide-tag measurements using a commercial kit](https://youtu.be/rd2G3yjWszQ?t=385)
2. [Slide-tag paper (Nature)](https://www.nature.com/articles/s41586-023-06837-4)
3. [Animation showing reverse transcriptase converting messenger RNA (mRNA) into complementary DNA (cDNA)](https://www.youtube.com/watch?v=SURGNo44wmU)
4. [Lecture on slide-tags by the group lead Dr. Fei Chen](https://www.youtube.com/watch?v=BlEUFdwUQfU)
5. [Supercooled water not freezing in rigid capillaries](https://www.youtube.com/watch?v=6c7JoCZmqC4)
6. [Microfluidics merging cells and beads](https://www.youtube.com/watch?v=IgIRoIMZN7Q)

[ this is a summary of a conversation with AI (gemini 2.5 pro) ]
