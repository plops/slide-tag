# A New Era in Genomics: Roche's Sequencing by Expansion (SBX) Technology

Roche has introduced a groundbreaking advancement in genomics with its proprietary Sequencing by Expansion (SBX) technology, heralding a new class of next-generation sequencing. This innovative method dramatically accelerates genomic analysis, reducing the time from a biological sample to a complete genome from days to mere hours. The technology stems from the pioneering work of Stratos Genomics, a company Roche acquired in 2020, and is poised to significantly impact genomic research and future clinical applications.

At the core of SBX is a novel biochemical process that transforms a target DNA or RNA molecule into a surrogate polymer, known as an "Xpandomer," which is approximately fifty times longer than the original strand. This expansion is achieved using specially engineered expandable nucleotides (XNTPs) and a unique polymerase. The elongated Xpandomer is then sequenced as it passes through a high-throughput nanopore sensor [9]. This fundamental innovation overcomes the signal-to-noise challenges inherent in conventional nanopore sequencing, allowing for highly accurate and efficient single-molecule reading.

The benefits of this new technology are multifaceted, offering a powerful combination of speed, accuracy, and flexibility. The system can generate over 5 billion duplex reads in a single hour, enough to sequence seven human genomes to 30x coverage, and has demonstrated a complete sample-to-variant-call workflow in under five hours. An initial analysis of publicly released data by bioinformatician Heng Li noted that the empirical base quality is comparable to Illumina's NovaSeq. SBX supports both high-accuracy duplex sequencing, ideal for sensitive applications like cancer research, and high-throughput simplex sequencing for RNA and single-cell analysis. Early collaborations with institutions like the Broad Institute and the Hartwig Medical Foundation are already showcasing its potential in real-world settings, such as providing rapid genomic insights for critically ill newborns in the neonatal intensive care unit (NICU). With a planned commercial launch for research use in 2026, Roche's SBX technology is set to redefine the standards of genomic sequencing.

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

# A First Look at Roche's SBX Sequencing Technology [4] (Harvard, h-index 81, was at Broad Institute in 2009)

This blog post provides an initial analysis of Roche's new short-read sequencing technology, Sequencing by Expansion (SBX), based on a recently released public dataset [8]. The author, Heng Li, examines the characteristics of the duplex SBX (SBX-D) data, noting its single-end reads of approximately 241bp. A key observation is the basecaller's method of assigning quality scores, which leads to 1bp insertions as the primary error type. While the empirical base quality is comparable to Illumina's NovaSeq, the author questions the decision to favor the longer strand in case of disagreements, as it may pose challenges for existing analysis tools. The post suggests alternative base-calling strategies that could make the data more compatible with current bioinformatics pipelines. Ultimately, the author concludes that the adoption of SBX will heavily depend on its pricing relative to established technologies.

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

**Abstract:**

This webinar introduces Roche's innovative Sequencing by Expansion (SBX) technology, a novel approach designed to overcome the limitations of direct DNA sequencing. The core of SBX is a biochemical process that converts a DNA molecule into an "expandimer," a larger surrogate molecule. This expanded structure allows for more accurate and efficient reading as it passes through a nanopore. The presentation details the complex chemistry involved, including specially engineered expandable nucleotides (XNTPs) and a modified polymerase called an XP synthase. The system utilizes a two-instrument setup for synthesis and sequencing, featuring a reusable high-throughput sensor array with 8 million pores.

Key performance data is presented, showcasing high accuracy (Q39 for duplex sequencing), massive throughput (over 5 billion duplex reads in one hour), and flexible read lengths. The webinar highlights two main sequencing approaches: high-accuracy duplex sequencing for applications like oncology and rapid whole genome sequencing, and high-throughput simplex sequencing for RNA and single-cell applications. Results from early access collaborations with the Hartwig Medical Foundation and the Broad Institute are shared, demonstrating the technology's potential in clinical research, including a sample-to-VCF time of under seven hours. The presentation also covers the data analysis pipeline, which is designed to handle the high data output in real-time. Finally, Roche outlines its commercialization strategy, with plans for a broader launch in 2026.

### **Roche's Sequencing by Expansion (SBX) Technology: A Breakthrough in Genomic Analysis**

*   **0:00:00 Introduction to Roche and the New Technology:** The webinar begins with a welcome and an overview of Roche's commitment to innovation in diagnostics. It sets the stage for the introduction of their new sequencing platform.
*   **0:03:22 The Inventor and the Core Concept:** Mark Kakoris, the inventor, is introduced. The fundamental idea behind SBX is to address the challenge of accurately reading DNA bases as they pass through a nanopore at high speed by first converting the DNA into a larger, expanded molecule called an "expandimer."
*   **0:05:15 Key Features of SBX Technology:** The technology is designed for flexibility, performance, and scalability. It aims to provide high accuracy, throughput, and variable read lengths while being cost-efficient.
*   **0:08:03 The Two-Instrument System:** The SBX system consists of two separate instruments: one for the SBX chemistry (synthesis) and another for the actual sequencing, which allows for greater flexibility in workflow.
*   **0:09:39 The "Expandimer": Not Sequencing DNA Directly:** The core of the technology is to not sequence DNA directly but to convert it into an "expandimer." This is a biochemical process that rescales the signal-to-noise challenges of direct DNA measurement.
*   **0:11:08 The Chemistry Behind Expansion:** This involves expandable nucleotide triphosphates (XNTPs), which have a tether attached. After incorporation by a specially engineered polymerase, this structure can be cleaved and expanded, replacing the original nucleotide with a new reporter code.
*   **0:13:45 The Engineered "XP Synthase":** A standard polymerase cannot handle the modified nucleotides. Roche has engineered a new enzyme, the "XP synthase," which is specifically designed to work with XNTPs.
*   **0:17:20 Controlled Movement Through the Nanopore:** A "translocation control element" is engineered into the expandimer. This allows the system to pause the molecule's movement and advance it one reporter code at a time using voltage pulses, ensuring a clean and reproducible signal.
*   **0:19:17 High-Density Sequencing Chip:** The technology utilizes a high-throughput sensor array with 8 million nanopores on a reusable chip, enabling massively parallel sequencing.
*   **0:24:26 Duplex vs. Simplex Sequencing:**
    *   **Duplex Sequencing:** Reads both parent strands of a DNA molecule, which are linked together on the same expandimer. This allows for intramolecular consensus, leading to extremely high accuracy (Q39) and is ideal for applications like somatic oncology.
    *   **Simplex Sequencing:** Reads a single strand, offering very high throughput for applications like RNA sequencing and single-cell analysis.
*   **0:26:16 Impressive Throughput and Accuracy:** In a one-hour run, the system generated 5.3 billion duplex reads, achieving over 30x coverage for seven human genomes with high F1 scores for variant calling.
*   **0:27:38 Early Access Collaborations:** Roche is working with the Hartwig Medical Foundation and the Broad Institute to test and validate the technology in real-world clinical research settings.
*   **0:31:19 The "SBXFAST" Protocol:** An amplification-free workflow that allows for a sample to be taken to variant call format (VCF) in approximately 5.5 hours. A demonstration at the Broad Institute achieved this in 6 hours and 25 minutes with just 20 minutes of sequencing.
*   **0:53:17 Real-Time Data Analysis:** The system is designed for accelerated local data processing to handle the massive data output (over 500 million bases per second for simplex reads). Base calling is done in real-time, with subsequent analysis steps also highly accelerated.
*   **0:57:46 High-Throughput Simplex Applications:** For RNA applications, the system can generate nearly 14 billion reads in one hour. It can also produce over 200 million reads longer than 1,000 bases in an hour for long-read applications.
*   **1:16:02 Commercialization Roadmap:** Roche plans to continue its early access program in 2025 and is targeting a commercial launch of the RUO (Research Use Only) product in 2026, with a future vision for clinical applications.

### **Glossary of Technical Terms**

*   **Sequencing by Expansion (SBX):** Roche's novel sequencing technology that involves biochemically converting a DNA molecule into a larger surrogate molecule (an expandimer) before sequencing.
*   **Expandimer:** The expanded surrogate molecule created from DNA. Its larger size makes it easier to read accurately as it passes through a nanopore.
*   **XNTP (eXpandable Nucleotide Triphosphate):** A modified nucleotide with a tether and reporter code attached. These are the building blocks used to create the expandimer.
*   **XP Synthase:** A specially engineered DNA polymerase designed to incorporate the bulky XNTPs into a growing strand, which a normal polymerase cannot do.
*   **Nanopore:** A tiny hole, typically in a membrane, through which the expandimer molecule is passed. Changes in the ionic current as the molecule moves through the pore are measured to determine the sequence.
*   **Duplex Sequencing:** A method where both strands of the original double-stranded DNA molecule are read. This allows for a consensus to be built, significantly increasing the accuracy of the final sequence.
*   **Simplex Sequencing:** A method where only a single strand of the DNA (or an RNA molecule) is sequenced. This approach maximizes throughput.
*   **VCF (Variant Call Format):** A standard text file format for storing gene sequence variations. The time it takes to go from a biological sample to a VCF file is a key benchmark for sequencing speed.
*   **F1 Score:** A measure of a test's accuracy. It considers both the precision and the recall of the test to compute the score. In this context, it is used to evaluate the accuracy of calling genetic variants.
*   **Throughput:** The amount of sequencing data that can be generated in a given amount of time.
*   **Read Length:** The length of the continuous sequence of bases that can be read from a single DNA molecule.
*   **Q-Score (Phred Quality Score):** A measure of the quality of the identification of the nucleobases generated by automated DNA sequencing. A Q-score of 30 (Q30) corresponds to a 1 in 1,000 chance of an incorrect base call (99.9% accuracy), while Q39 corresponds to higher accuracy.


# eshg-2025-workshop-mark-kokoris [6]

**Abstract:**

This presentation provides a comprehensive update on a novel single-molecule sequencing technology, Sequencing by Expansion (SBX), which converts DNA into a larger surrogate molecule, an "expandomer," for high-resolution analysis via a nanopore sensor array. The speaker highlights significant performance improvements in both accuracy and speed, detailing the achievement of Q40 average base quality with the high-fidelity duplex sequencing method and a sample-to-variant-call workflow completed in under five hours. The talk introduces the first-ever datasets for challenging applications, demonstrating the platform's potential in analyzing damaged DNA from FFPE samples and its high sensitivity in detecting Minimal Residual Disease (MRD) from low-input cell-free DNA. Furthermore, the presentation covers the high-throughput simplex sequencing mode, showcasing its ability to generate a terabase of long-read data in a single hour for applications such as genomic phasing.

**Advancements in Sequencing by Expansion (SBX): New Data on Speed, Accuracy, and Clinical Applications**

*   **[0:55] Technology Update:** The presentation aims to update on the SBX duplex and fast sequencing methods, introduce the first-ever data for FFPE and Minimal Residual Disease (MRD) applications, and discuss simplex applications for RNA and DNA.
*   **[1:53] Core Technology:** The platform combines "Sequencing by Expansion" (SBX) chemistry with a high-throughput nanopore sensor. It avoids sequencing DNA directly by first creating a larger surrogate molecule called an "expandomer," which improves the signal-to-noise ratio during measurement.
*   **[4:47] Expandable Nucleotides (XNTPs):** The chemistry is built on specially engineered nucleotides (XNTPs) that contain elements to control the molecule's movement through the nanopore and enhance their incorporation, which is key to the process.
*   **[6:57] Controlled Measurement:** The system uses voltage pulses to move the expandomer through the nanopore one base at a time. This deterministic control allows for high-quality, efficient, and flexible data acquisition, from short runs of a few minutes to longer runs of several hours.
*   **[8:41] Massively Parallel Sequencing:** The sequencing occurs on a chip containing an array of 8 million microwells, each with a nanopore, enabling massive throughput. The chip is designed to be reusable to help drive down costs.
*   **[11:37] Duplex Sequencing for High Accuracy:** The SBX Duplex (SBX-D) method uses a hairpin adapter to sequence both strands of the original DNA molecule. By creating an intramolecular consensus, this method achieves very high accuracy.
*   **[13:46] Improved Accuracy:** The platform has improved its average duplex base quality to Q40, which corresponds to 99.99% accuracy. F1 scores for calling variants have significantly improved in just a few months.
*   **[16:46] SBX Fast Workflow:** An amplification-free version of the duplex workflow, called SBX Fast, demonstrates even better performance on difficult homopolymer regions (stretches of identical bases).
*   **[19:05] Sub-5-Hour Genome:** Using the SBX Fast workflow, the total time from sample preparation to a final variant call file (VCF) has been reduced to 4 hours and 23 minutes, a significant speed improvement.
*   **[21:27] First FFPE Data:** The presentation showcases the first application of the SBX-D workflow to challenging Formalin-Fixed Paraffin-Embedded (FFPE) tissue samples, which often contain damaged DNA. The results indicate lower error rates and better performance in high GC-content regions compared to other technologies.
*   **[26:07] Minimal Residual Disease (MRD) Detection:** In another first, the SBX-D protocol was used for MRD analysis, successfully detecting cancer-related mutations from just 4 nanograms of cell-free DNA.
*   **[27:18] High MRD Sensitivity:** The platform demonstrated the ability to detect MRD in all 15 cancer samples tested, with an estimated sensitivity down to a tumor fraction of one in a million (10⁻⁶).
*   **[28:00] Simplex Sequencing for Long Reads:** The SBX Simplex (SBX-S/SL) mode reads a single strand of DNA, enabling much higher throughput and longer read lengths (SL denotes reads >500 bases).
*   **[30:42] Terabase-Scale Phasing:** A one-hour sequencing run in simplex mode generated one billion reads with an average length of nearly 1,000 bases. This volume of long-read data is powerful for genomic phasing, which determines which parent a genetic variant came from.
*   **[33:51] Future Work:** The team will continue to optimize the technology and expand into new applications, including targeted enrichment and methylation analysis.

**Glossary of Technical Terms**

*   **SBX (Sequencing by Expansion):** The core technology that converts a DNA molecule into a larger surrogate molecule ("expandomer") to enable more accurate single-molecule sequencing.
*   **Expandomer:** The synthetic polymer molecule created from a DNA template, designed for controlled, high-resolution passage through a nanopore.
*   **XNTP (Expandable Nucleotide):** A modified nucleotide, the chemical building block used to synthesize an expandomer.
*   **Duplex Sequencing:** A high-accuracy sequencing method where both strands of a double-stranded DNA molecule are sequenced and compared to create a highly accurate consensus sequence.
*   **Simplex Sequencing:** A sequencing method that reads only a single strand of a DNA molecule, typically allowing for higher throughput and longer reads than duplex sequencing.
*   **FFPE (Formalin-Fixed Paraffin-Embedded):** A standard method of preserving biological tissue samples that can cause chemical damage to DNA, making it challenging to sequence accurately.
*   **MRD (Minimal Residual Disease):** Refers to the small number of cancer cells that can remain in a patient after treatment. Detecting MRD requires highly sensitive sequencing methods.
*   **VCF (Variant Call Format):** A standardized text file format used in bioinformatics for storing information about genetic sequence variations.
*   **F1 Score:** A statistical measure of a test's accuracy, calculated from its precision and recall. A score of 1 indicates perfect accuracy.
*   **Homopolymer:** A repetitive sequence in a DNA strand consisting of a series of identical bases (e.g., AAAAAA). These can be challenging for many sequencing technologies to read correctly.
*   **Nanopore:** A nano-scale pore, typically a protein embedded in a membrane, through which a single molecule (like an expandomer) is passed for analysis.
*   **Q Score (Phred Quality Score):** A numerical score that represents the accuracy of a sequenced DNA base. Q40 indicates a 1 in 10,000 chance of an incorrect base call (99.99% accuracy).


# sean-hofherr-eshg-2025-workshop-mc [7] (Broad Clinical Labs)

**Abstract**

This presentation details the collaboration between Broad Clinical Labs and Roche to transition a novel, high-speed, high-accuracy Nanopore and Expandimer sequencing technology from a proof-of-principle concept to a practical clinical tool. The primary application explored is rapid whole-genome sequencing for trio testing (mother, father, and infant) in the neonatal intensive care unit (NICU). The speaker highlights the significant reduction in sequencing time, achieving results for three genomes in just over seven hours, a critical advancement for diagnosing acutely ill newborns where time is of the essence. The technology has successfully identified a wide range of pathogenic variants, including small nucleotide variants, indels, large copy number events, and repeat expansions, in patient cell lines. Furthermore, the data generated is compatible with existing tertiary analysis software, demonstrating a viable workflow from sequencing to a potential clinical report. The ongoing collaboration aims to further optimize, automate, and validate this end-to-end workflow to implement one-day, one-shift rapid genome analysis for patient care.

**From Concept to Clinic: Rapid Trio-Genomic Sequencing**

*   **0:26 A Mission to Translate Technology:** The Broad Institute's Clinical Labs focuses on applying large-scale genomic technology to solve clinical problems, working with a diverse range of partners from research to clinical translation.
*   **1:51 Platform Agnostic Sequencing Powerhouse:** The lab is equipped with a massive fleet of sequencers, including NovaSeq X Pluses, Ultima systems, and PacBio Revios, positioning it as a proving ground for new technologies.
*   **3:31 Targeting a Critical Need:** The collaboration with Roche focuses on rapid genome sequencing for newborns in the NICU. Early diagnosis in these cases can significantly reduce the burden on patients, families, and the healthcare system.
*   **4:57 Redefining "Rapid" Testing:** While current "rapid" testing can take from 72 hours to a week, the goal is to provide diagnoses faster, as every hour matters for critically ill infants.
*   **5:49 Initial Success and Impressive Speed:** Early test runs of the Roche system sequenced three samples in 12 hours and 10 minutes. Subsequent optimizations dramatically reduced this time, achieving a single genome sequence in as little as four hours and 25 minutes.
*   **8:15 Focusing on Clinical Reality:** The ultimate goal is a production-level workflow. The most clinically useful application is seen as trio testing (mother, father, and child) to determine if a variant is newly acquired or inherited.
*   **9:26 Breakthrough in Trio Sequencing:** The team successfully sequenced three whole genomes simultaneously in seven hours and eight minutes, consistently achieving over 30x coverage.
*   **9:50 Validated Diagnostic Accuracy:** Testing on patient cell lines with known conditions (like inborn errors of metabolism, cystic fibrosis, and muscular dystrophy) confirmed the system's ability to detect a wide array of causative variants, including single nucleotide variants, deletions, complex indels, copy number events, and repeat expansions.
*   **12:28 Seamless Software Integration:** Data from the new sequencer was successfully loaded into the Fabric Genomics tertiary analysis platform, which is the same system Broad Clinical Labs uses for its standard clinical work. This test demonstrated that causative variants could be prioritized correctly without any special adjustments to the software.
*   **14:11 Future Goals:** The collaboration is now focused on further optimizing bioinformatics, streamlining the workflow through automation, validating performance across various tertiary analysis platforms, and ultimately, implementing the technology for real-world patient care to achieve a one-day, one-shift turnaround for rapid genome results.

**Glossary**

*   **Copy Number Event (CNV):** A type of genetic variation where a segment of DNA is present in a different number of copies than the standard two (one from each parent).
*   **CV (Curriculum Vitae):** A detailed document highlighting a person's professional and academic history.
*   **De Novo Variant:** A genetic alteration that is present for the first time in one family member as a result of a mutation in a germ cell (egg or sperm) of one of the parents or in the fertilized egg itself.
*   **Heterozygous:** Having two different alleles (variants) for a particular gene.
*   **Homozygous:** Having two identical alleles for a particular gene.
*   **Homopolymers:** A stretch of DNA consisting of repeated instances of the same nucleotide (e.g., AAAAAA).
*   **IGV (Integrative Genomics Viewer):** A high-performance visualization tool for exploring large, integrated genomic datasets.
*   **Indels:** A type of genetic variation that involves the insertion or deletion of nucleotides in the DNA sequence.
*   **NICU (Neonatal Intensive Care Unit):** A specialized hospital unit that provides intensive care for ill or premature newborn infants.
*   **Proband:** The first person in a family to receive genetic counseling or testing for a suspected hereditary condition.
*   **Repeat Expansion:** A type of mutation where a specific short sequence of DNA is repeated multiple times in a row.
*   **SMVs (Small Nucleotide Variants):** Changes to a single nucleotide in the DNA sequence.
*   **Tertiary Analysis:** The process of interpreting the clinical significance of genetic variants found during sequencing, often involving software to filter and prioritize variants based on medical literature, population data, and predictive algorithms.
*   **VCF (Variant Call Format):** A standard text file format for storing gene sequence variations.

# Introducing Roche Sequencing by Expansion (SBX) Technology [9] (Amazing visualization of the Expandomere function)


*Abstract:*

This video explains Roche's innovative Sequencing by Expansion (SBX) technology, a method designed to overcome the physical limitations of resolving closely spaced DNA bases. The technology synthesizes a surrogate molecule, called an "expander," which is over 50 times longer than the original DNA template. This is achieved using novel nucleotides (xNTPs), each carrying a large, high-signal "reporter" that corresponds to one of the four DNA bases. After synthesis, the expander is threaded through a biological nanopore. A Translocation Control Element (TCE) within each reporter momentarily holds it in the pore, generating a distinct electrical signal that identifies the base. A voltage pulse then advances the expander to the next reporter. This process is massively parallelized on a CMOS sensor, enabling extremely high, real-time sequencing rates.

*Roche's Sequencing by Expansion (SBX): A Nanopore-Based Approach*

*   *0:00:06 The Fundamental Challenge:* DNA bases are separated by only 3.4 angstroms, a distance too small for many sequencing methods to resolve easily.
*   *0:00:16 The SBX Solution:* Sequencing by Expansion (SBX) technology creates a physically expanded surrogate molecule, the "expander," which is over 50 times longer than the target DNA, making it much easier to read.
*   *0:00:38 Novel Building Blocks (xNTPs):* The expander is built from four types of novel nucleotides called xNTPs (A, C, G, and T).
*   *0:00:46 Expander Synthesis:* Using a modified DNA replication process, xNTPs are linked sequentially along the target DNA template, creating a new molecule that encodes the original DNA sequence information into its structure.
*   *0:01:04 High-Signal Reporters:* Each xNTP contains a large, high signal-to-noise "reporter" molecule attached via a tether. The identity of the reporter mirrors the identity of the DNA base it represents.
*   *0:01:20 Translocation Control Element (TCE):* Located in the center of each reporter, the TCE is a key component that precisely modulates the expander's movement through the nanopore during the sequencing phase.
*   *0:01:55 Creating the Expander:* After synthesis, a reagent degrades the original DNA template and cleaves a bond in the xNTP backbone, allowing the molecule to physically expand.
*   *0:02:11 Electrical Reading via Nanopore:* The expander is threaded through a biological nanopore. The TCE holds each reporter in the pore, altering the electrical resistance to generate one of four unique signals that identifies the corresponding DNA base.
*   *0:02:37 Step-Wise Advancement:* After a base is read, a short, high-voltage pulse is applied to reliably advance the expander to the next reporter for measurement.
*   *0:02:46 Massive Parallelization and Speed:* This process is performed simultaneously across millions of wells on a single CMOS-based sensor, achieving remarkable real-time sequencing rates of hundreds of millions of bases per second.

# Roche SBX Sequencing: Game-Changer or Just Hype? [10] (Short Review by independent Biologist)

*Abstract:*

This video provides a concise summary of Roche's new sequencing technology, SPX (Sequencing by Expansion), based on the company's 82-minute webinar [2]. The core of the technology involves synthesizing a stretchable, accordion-like DNA analog called an "expandimemer" and reading its sequence by threading it through a pore, similar to nanopore sequencing. Key differentiators are that SPX is a short-read technology and incorporates "translocation control elements" in the expandimemer's backbone to ensure controlled, base-by-base movement through the pore.

The primary advantage of SPX is its exceptional speed, enabled by a high-density flow cell containing 8 million pores. This allows for generating 5 billion reads in just one hour and completing an entire workflow, from library preparation to variant calling, in under 6.5 hours. In terms of accuracy, SPX achieves Q39 scores and is reported to be roughly on par with Illumina for SNP and indel calling. While cost details are not yet available, the technology is expected to be competitive with existing platforms. The platform is anticipated to launch in 2026.

*Key Takeaways on RO's SPX Sequencing Technology*

*   *00:00:10 What is SPX?:* SPX stands for "Sequencing by Expansion." It works by creating a complimentary, stretchable DNA analog ("expandimemer") which is then threaded through a pore. The sequence is read by detecting changes in electrical current, a method similar to nanopore sequencing.
*   *00:00:29 SPX vs. Nanopore:* While the detection method is similar, SPX differs significantly. It is a short-read technology (inserts in the low hundreds of base pairs) and features "translocation control elements" for a more controlled, paused movement between each base reading.
*   *00:00:58 Main Advantage - Speed:* The technology is exceptionally fast. It can produce 5 billion reads in one hour (or 15 billion in four hours) and complete a full workflow from library prep to variant calling in under 6.5 hours.
*   *00:01:16 High-Density Flow Cell:* Its speed is attributed to a flow cell with 8 million pores, which is orders of magnitude higher than the tens of thousands of pores found in the highest-throughput nanopore instruments.
*   *00:01:28 High Accuracy:* SPX demonstrates high accuracy, achieving Q39 quality scores. For SNP and indel calling, its accuracy is in the 99% to 99.8% range, making it competitive with Illumina platforms.
*   *00:01:53 Cost is Unknown:* RO has not yet announced pricing for the instrument or its reagents, but it is expected to be competitive with platforms like the Illumina NovaSeq on a cost-per-gigabyte basis.
*   *00:02:24 A Key Limitation:* A current drawback is that the system cannot analyze native DNA or RNA directly. It requires the conversion of the sample into the synthetic "expandimemer" molecule first.
*   *00:02:32 Expected Launch:* The SPX platform is scheduled for a commercial launch in 2026.


# References
- 1 https://www.roche.com/media/releases/med-cor-2025-02-20
- 2 https://www.roche.com/investors/events/roche-virtual-event-on-the-sbx-technology
- 3 https://sequencing.roche.com/us/en/article-listing/sequencing-platform-technologies.html
- 4 https://lh3.github.io/2025/09/11/a-quick-look-at-roches-sbx 11 September 2025
- 5 https://sequencing.roche.com/us/en/videos/webinar-sequencing-by-expansion-technology.html Sequencing by Expansion (SBX) webinar September 10, 2025
- 6 https://roche.scene7.com/is/content/RocheDiaProd/eshg-2025-workshop-mark-kokoris-mc--17411
- 7 https://roche.scene7.com/is/content/RocheDiaProd/sean-hofherr-eshg-2025-workshop-mc--17449
- 8 https://sequencing.roche.com/global/en/events/sbx-d-data-analysis-webinar.html
- 9 Introducing Roche Sequencing by Expansion (SBX) Technology https://www.youtube.com/watch?v=G8ECt04qPos 20 February 2025
- 10 Roche SBX Sequencing: Game-Changer or Just Hype? https://www.youtube.com/watch?v=rbIGMYfXjdo 26 Mar 2025
