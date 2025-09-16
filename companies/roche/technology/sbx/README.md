# Summarizing Roche's Sequencing by Expansion (SBX) Technology [1]

Roche has unveiled its breakthrough Sequencing by Expansion (SBX) technology, introducing a new class of next-generation sequencing. This innovative approach combines proprietary SBX chemistry with a high-throughput CMOS-based sensor module to deliver ultra-rapid, scalable, and flexible genomic insights. By encoding target nucleic acid sequences into fifty-times longer "Xpandomers" for highly accurate single-molecule nanopore sequencing with superior signal-to-noise, SBX significantly reduces the time from sample to genome from days to hours. This advancement is poised to revolutionize genomic research and holds substantial promise for future translational and clinical applications in understanding complex diseases like cancer, immune, and neurodegenerative disorders. The technology originated from Stratos Genomics, acquired by Roche in 2020.

*   *New Sequencing Class:* Roche announced its proprietary Sequencing by Expansion (SBX) technology on February 20, 2025, establishing a new category of next-generation sequencing (NGS).
*   *Addressing Complex Diseases:* SBX is designed to provide detailed genetic insights, crucial for decoding complex diseases such as cancer, immune disorders, and neurodegenerative conditions.
*   *Core Innovation:* The technology integrates SBX chemistry with an advanced, high-throughput Complementary Metal Oxide Semiconductor (CMOS)-based sensor module.
*   *How it Works (Xpandomers):* SBX encodes the sequence of a target nucleic acid (DNA or RNA) into a measurable "Xpandomer" polymer, which is fifty times longer than the original molecule. These Xpandomers provide high signal-to-noise reporters for accurate single-molecule nanopore sequencing.
*   *Key Benefits:*
    *   *Ultra-rapid:* Reduces the time from sample to genome from days to hours, significantly speeding up genomic research.
    *   *Scalable & Flexible:* Capable of operating across a range of throughput scales, suitable for various project sizes from small studies to large-scale initiatives.
    *   *High Accuracy:* Achieved through clear signals and minimal background noise, a key efficiency driver for the technology.
*   *Broad Applications:* SBX is versatile for whole genome sequencing, whole exome sequencing, and RNA sequencing, with potential for adoption in both research laboratories and future clinical settings.
*   *Leadership Vision:* Matt Sause, CEO of Roche Diagnostics, emphasizes SBX's unparalleled speed, efficiency, and flexibility, foreseeing its potential to revolutionize sequencing in research and healthcare.
*   *Origin of SBX:* The chemistry was invented by Mark Kokoris and Robert McRuer, co-founders of Stratos Genomics, which Roche acquired in 2020.
*   *Complementary Portfolio:* Roche's existing sequencing portfolio includes KAPA sample preparation products and the AVENIO Edge system for automation, along with AVENIO assays for oncology, which play a significant role in the sequencing ecosystem.

# A First Look at Roche's SBX Sequencing Technology [4]

This blog post provides an initial analysis of Roche's new short-read sequencing technology, Sequencing by Expansion (SBX), based on a recently released public dataset. The author, Heng Li, examines the characteristics of the duplex SBX (SBX-D) data, noting its single-end reads of approximately 241bp. A key observation is the basecaller's method of assigning quality scores, which leads to 1bp insertions as the primary error type. While the empirical base quality is comparable to Illumina's NovaSeq, the author questions the decision to favor the longer strand in case of disagreements, as it may pose challenges for existing analysis tools. The post suggests alternative base-calling strategies that could make the data more compatible with current bioinformatics pipelines. Ultimately, the author concludes that the adoption of SBX will heavily depend on its pricing relative to established technologies.

*   **Introduction to SBX:** Roche has introduced a new short-read sequencing technology called Sequencing by Expansion (SBX). A public dataset from duplex SBX (SBX-D) was recently released.
*   **Data Characteristics:** The initial public data consists of single-end reads with an average length of about 241 base pairs. Over 95% of these reads are longer than 150bp.
*   **Duplex Sequencing Insights:** With SBX-D, the sequencing of the complementary strand starts from the 3'-end of the first strand. This results in the 3'-ends being mostly duplex, while the 5'-ends are often covered by only a single strand.
*   **Quality Score Assignment:** The current SBX-D basecaller assigns different quality scores based on strand agreement: a score of 39 for bases supported by both strands, 5 for bases where the strands disagree, and 22 for bases covered by only one strand.
*   **Primary Error Profile:** The main type of error observed in SBX-D reads are 1-base-pair insertions. This is a direct consequence of the basecaller's decision to always select the base from the longer strand when the two strands are in disagreement.
*   **Comparable Base Quality:** The empirical base quality of SBX-D is around Q21 and remains consistent across the reads. For bases that are confirmed by both strands, the quality reaches Q37, which is similar to the performance of the Illumina NovaSeq.
*   **Concerns About Current Methodology:** The author expresses concern over the basecaller's strategy of always choosing the longer strand. This approach could create issues for many existing analysis tools that are based on pileups.
*   **Proposed Improvements:** The author suggests that using the raw signal data could help generate a more accurate duplex consensus. This would make SBX-D data more compatible with standard short-read analysis pipelines. He also proposes a machine-learning model to classify simplex bases and make more informed decisions during duplex consensus.
*   **The Importance of Pricing:** The future of SBX and its adoption by the research community will likely be heavily influenced by its cost. If it is priced competitively against existing technologies like NovaSeq, it could drive a shift in tool development to better support its unique data characteristics.

**Glossary of Technical Terms**

*   **Basecaller:** A program that processes the raw signal output from a DNA sequencing instrument to determine the sequence of nucleotide bases.
*   **DeepVariant:** A deep learning-based tool developed by Google for calling genetic variants from next-generation sequencing data with high accuracy.
*   **Duplex SBX (SBX-D):** A method within Roche's Sequencing by Expansion technology that involves sequencing both strands of a DNA molecule to improve accuracy.
*   **emQ (Empirical Base Quality):** A measure of the accuracy of base calls in sequencing data, determined by comparing the sequencing reads to a known reference sequence.
*   **GATK (Genome Analysis Toolkit):** A widely used set of software tools for analyzing high-throughput sequencing data, with a focus on variant discovery.
*   **Homopolymer Run:** A sequence of three or more identical consecutive nucleotide bases in a DNA sequence.

# Sequencing by Expansion (SBX) webinar [5]

# eshg-2025-workshop-mark-kokoris [6]

# sean-hofherr-eshg-2025-workshop-mc [7]

# References
- 1 https://www.roche.com/media/releases/med-cor-2025-02-20
- 2 https://www.roche.com/investors/events/roche-virtual-event-on-the-sbx-technology
- 3 https://sequencing.roche.com/us/en/article-listing/sequencing-platform-technologies.html
- 4 https://lh3.github.io/2025/09/11/a-quick-look-at-roches-sbx 11 September 2025
- 5 https://sequencing.roche.com/us/en/videos/webinar-sequencing-by-expansion-technology.html Sequencing by Expansion (SBX) webinar September 10, 2025
- 6 https://roche.scene7.com/is/content/RocheDiaProd/eshg-2025-workshop-mark-kokoris-mc--17411
- 7 https://roche.scene7.com/is/content/RocheDiaProd/sean-hofherr-eshg-2025-workshop-mc--17449
