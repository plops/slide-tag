# **A Guide to the Slide-tags Dataset and Computational Workflow**

This document provides a detailed description of the input data, source code, and output files for the Slide-tags spatial genomics pipeline. It uses the Trekker Mouse Kidney dataset as a practical example to illustrate the structure of the data and the steps involved in transforming raw sequencing reads into a spatially-resolved map of nuclei.

### **1. Input Data Structure: The Mouse Kidney Example**

The processing pipeline requires three distinct types of input data: the raw sequencing reads from the spatial barcode (SB) library, the official map of bead barcode coordinates for the slide used, and the processed gene expression data from a standard single-nucleus RNA-seq workflow (e.g., 10x Genomics Cell Ranger).

The file listing below shows the typical structure of an input directory:

```
└── [4.0K May  7 00:36]  TrekkerU_RATAC_ExampleInput_MouseKidney1
    ├── [895M May  7 00:34]  TrekkerU_RATAC_MouseKidney1_R1_001.fastq.gz
    ├── [710M May  7 00:34]  TrekkerU_RATAC_MouseKidney1_R2_001.fastq.gz
    ├── [ 24M May  7 00:33]  U0027_016_BeadBarcodes.txt
    └── [4.0K May  9 18:16]  TrekkerU_RATAC_MouseKidney1_scRNAseqOut
        ├── [4.0K May  9 18:30]  TrekkerU_RATAC_MouseKidney1_ATAC_Cell_by_Peak_MEX
        │   ├── [596M May  9 18:18]  ATAC_Fragments.bed.gz
        │   ├── [625K May  9 18:18]  ATAC_Fragments.bed.gz.tbi
        │   ├── [ 39K May  7 00:53]  atac-barcodes.tsv.gz
        │   ├── [1.5M May  7 00:53]  atac-features.tsv.gz
        │   └── [ 45M May  7 00:53]  atac-matrix.mtx.gz
        └── [4.0K May  9 18:43]  TrekkerU_RATAC_MouseKidney1_RSEC_MolsPerCell_MEX
            ├── [ 39K May  7 00:35]  barcodes.tsv.gz
            ├── [148K May  7 00:35]  features.tsv.gz
            ├── [ 13M May  7 00:35]  matrix.mtx.gz
            └── [377K May  9 18:43]  translate.csv
```

#### **Key Input Files:**

1.  **Raw Sequencing Reads (FASTQ):** These are standard paired-end sequencing files (`.fastq.gz`). For the spatial barcode library, they contain both the spatial barcode sequences and the associated 10x Genomics cell barcode sequences.
    *   `TrekkerU_RATAC_MouseKidney1_R1_001.fastq.gz`
    *   `TrekkerU_RATAC_MouseKidney1_R2_001.fastq.gz`

    *Excerpt from a FASTQ file:*
    ```
    @VL00269:355:AAG5VW3M5:1:1101:18989:1000 1:N:0:CGTACTAN+TATGCAGT
    AGTTATCACCGTGATTGTGCGGAGACAATTAAGCATATCTGCCTTTTTTTTTTTTTTTTTTTTTTTTCC
    +
    IIIIIIII9IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII9IIIIIIIIIIIII
    ...
    ```

2.  **Bead Barcode Map:** This text file (`U0027_016_BeadBarcodes.txt`) is the crucial "answer key." It contains the definitive list of all spatial barcode sequences present on the slide and their corresponding physical (x, y) coordinates.

    *Excerpt from `BeadBarcodes.txt`:*
    ```
    CAGGTGCCAAATAG  1942.0  594.4
    CATTCCATCGTCTA  2338.6  1531.6
    ...
    ```

3.  **Single-Cell Analysis Output (Cell Ranger):** The directory `TrekkerU_RATAC_MouseKidney1_scRNAseqOut` contains the processed gene expression data. The most critical file for this pipeline is the list of valid cell barcodes (e.g., `barcodes.tsv.gz`), which identifies all the nuclei that passed the initial single-cell quality control.

    *Excerpt from `barcodes.tsv.gz`:*
    ```
    13236
    19637
    19697
    ...
    ```

### **2. The Data Processing Pipeline and Source Code**

The source code, available on GitHub at `broadchenf/Slide-tags`, orchestrates the transformation of the input data into a final spatial map. The workflow consists of four main steps executed by a series of shell, R, and Python scripts.

```
└── broadchenf/Slide-tags
    ├── [1.3K]  README.md
    ├── [6.3K]  bead_matching.py
    ├── [7.7K]  cell_barcode_matcher.R
    ├── [2.2K]  sb_processing.sh
    └── [ 33K]  spatial_positioning.R
```

#### **Step 1: FASTQ Pre-processing (`sb_processing.sh`)**
This initial shell script filters the massive raw FASTQ files to isolate only the reads relevant to the spatial analysis.

*   **Function:** It scans the R2 file for a specific DNA anchor sequence ("TCTTCAGCGTTCCCGAGA") that flanks the spatial barcode. Only reads containing this sequence are kept.
*   **Downsampling:** Because spatial libraries are often sequenced to a very high depth, the script then uses the `seqtk` toolkit to randomly downsample the filtered reads to a manageable number (e.g., 25 million). This significantly reduces the computational load of subsequent steps without sacrificing accuracy.

#### **Step 2: Matching Spatial and Cell Barcodes (`cell_barcode_matcher.R`)**
This R script is responsible for creating the fundamental link between the spatial barcode information and the cell identity.

*   **Function:** It parses the pre-processed FASTQ reads to extract the spatial barcode (SB) and the 10x cell barcode (CB) from each read. It then uses the list of valid cell barcodes from the Cell Ranger output as a whitelist to create a definitive table of all observed SB-CB pairings.

#### **Step 3: Assigning Coordinates to Spatial Barcodes (`bead_matching.py`)**
This Python script translates the sequenced spatial barcodes into their physical locations on the slide.

*   **Function:** It takes the list of all unique spatial barcodes found in the sequencing data and matches them against the official coordinate map (`BeadBarcodes.txt`).
*   **Error Correction:** The script employs both exact matching and "fuzzy" matching (allowing for a small number of sequence mismatches, or a low "edit distance") to robustly account for potential sequencing errors.

#### **Step 4: Spatial Mapping of Nuclei (`spatial_positioning.R`)**
This final R script is the core of the spatial reconstruction. It takes all the spatial barcode locations associated with each cell and calculates a single, precise (x, y) coordinate for that nucleus.

*   **Clustering with DBSCAN:** For each cell, the pipeline uses the DBSCAN (Density-Based Spatial Clustering of Applications with Noise) algorithm.
    1.  **Point Cloud Generation:** It gathers the (x, y) coordinates of all spatial barcodes associated with a single cell, creating a 2D point cloud.
    2.  **Cluster Identification:** Ideally, these points should form a single dense cluster corresponding to the original location of the nucleus. DBSCAN excels at identifying this primary cluster while simultaneously classifying outlier points (experimental noise) that are far from the dense region.
    3.  **Weighted Centroid Calculation:** Instead of a simple average, the script calculates a **UMI-weighted centroid** of the primary cluster. The coordinates of spatial barcodes that were detected more frequently (higher UMI counts) are given more weight, resulting in a more accurate and robust final position.
*   **Parameter Optimization:** The script automatically tests a range of DBSCAN parameters (specifically `minPts`) and selects the optimal value that maximizes the number of confidently positioned nuclei (i.e., those that yield a single, unambiguous spatial cluster).

### **3. Understanding the Output Data**

The pipeline generates a comprehensive output directory containing the final results, intermediate files for quality control, summary reports, and diagnostic plots.

```
└── [4.0K May 12 20:34]  trekker_TrekkerU_RATAC_MouseKidney1
    ├── [4.0K May  7 00:13]  misc
    │   ├── [ 22M May  7 00:13]  BeadBarcodes.txt
    │   ├── [ 32M May  7 00:13]  matching_result_TrekkerU_RATAC_MouseKidney1.csv
    │   └── ... (other intermediate files)
    └── [4.0K May 12 20:02]  output
        ├── [ 10M May  7 00:13]  TrekkerU_RATAC_MouseKidney1_ConfPositioned_anndata_matched.h5ad
        ├── [347M May 12 20:28]  TrekkerU_RATAC_MouseKidney1_ConfPositioned_seurat_spatial.rds
        ├── [293K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_Location_ConfPositionedNuclei.csv
        ├── [ 32M May  7 00:13]  TrekkerU_RATAC_MouseKidney1_MoleculesPer_ConfPositionedNuclei.mtx
        ├── [ 12M May  7 00:13]  TrekkerU_RATAC_MouseKidney1_Trekker_Report.html
        ├── [158K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_barcodes_ConfPositionedNuclei.tsv
        ├── [209K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_genes_ConfPositionedNuclei.tsv
        ├── [2.3K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_summary_metrics.csv
        ├── [4.0K May  7 00:13]  cell_bc_plots
        │   ├── [4.0K May  7 00:13]  cells_1_coordinates_assigned
        │   │   ├── [446K May  7 00:13]  AGCGGCCAGTTGGTTAATTATTGCGCA.jpeg
        │   │   └── ... (images for each cell)
        └── [4.0K May  7 00:13]  plots
            └── [ 864 May  7 00:13]  summary_metrics.csv
```

#### **3.1 Primary Results (in `output/`)**

This directory contains the main data files ready for downstream biological analysis.

*   **Spatial Coordinates:** The most important output is `...Location_ConfPositionedNuclei.csv`. This file provides the final calculated (x, y) coordinates for each confidently mapped cell barcode.

    *Excerpt from `...Location_ConfPositionedNuclei.csv`:*
    ```
    ,SPATIAL_1,SPATIAL_2
    TGTGTTCGCTCACACTTATCCATTCAT,4627.2793,-3301.5116
    TGTGTTCGCTAGTTCAATGGTCCGACT,7811.34746,-6443.133325
    ...
    ```

*   **Spatially-Filtered Expression Matrix:** The files `...MoleculesPer_ConfPositionedNuclei.mtx`, `...barcodes_ConfPositionedNuclei.tsv`, and `...genes_ConfPositionedNuclei.tsv` together form the gene expression matrix. Crucially, this matrix has been filtered to contain data *only* for the nuclei that were successfully assigned a spatial coordinate.

*   **Integrated Analysis Objects:** For convenience, the pipeline provides pre-packaged data objects for common analysis toolkits:
    *   `...ConfPositioned_seurat_spatial.rds`: An R object for the Seurat package.
    *   `...ConfPositioned_anndata_matched.h5ad`: A Python AnnData object for Scanpy and Squidpy.

*   **HTML Report:** The `...Trekker_Report.html` file provides a summary of the analysis, including key quality control metrics and visualizations.

#### **3.2 Intermediate and QC Files (in `misc/`)**

This directory stores intermediate outputs that are valuable for troubleshooting and quality control. Key files include `matching_result_...csv`, which details the mapping between sequenced SBs and the official bead barcodes, and `matcher_summary_...txt`, which gives high-level statistics on matching efficiency.

#### **3.3 Visual Quality Control and Plots (in `output/cell_bc_plots/` and `output/plots/`)**

These directories provide graphical summaries of the pipeline's performance.

*   **Per-Cell Plots (`cell_bc_plots/`):** This contains JPEG images for individual cells, showing the 2D distribution of the spatial barcode reads associated with them. Subdirectories like `cells_1_coordinates_assigned` group cells by the number of DBSCAN clusters found, providing direct visual evidence of positioning quality. An ideal cell will have a single, tight cluster of points.
*   **Summary Plots (`plots/`):** This contains figures and tables, such as `summary_metrics.csv`, that summarize the performance of the DBSCAN clustering and the results of its parameter optimization.