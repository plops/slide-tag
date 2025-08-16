[ This file was generated with the help of AI ]

# Accessing Takara Bio Trekker / Slide-tags Datasets and Source Code

Researchers interested in utilizing the Takara Bio Trekker / Slide-tags technology for spatially resolved single-cell
analysis can access both example datasets and the underlying source code to facilitate their work.

## Example Datasets:

Takara Bio provides a collection of datasets generated with the Trekker Single-Cell Spatial Mapping Kit. These datasets
span various human and mouse tissues, including mouse brain, embryo, and kidney, as well as human breast cancer and
melanoma.[1] While most of these are output files, some raw sequencing data in FASTQ format are also available for
download.[1] Access to these datasets requires registration on the Takara Bio website, after which login credentials
will be provided via email.[1]
These example datasets allow researchers to explore the capabilities of the Trekker and its associated analysis
pipelines without having to generate their own data initially.[1] The Trekker system is designed to be species-agnostic
and compatible with various single-cell platforms like the 10x Genomics Chromium and BD Rhapsody systems.[1][2]


## Source Code:

The computational pipeline for assigning spatial coordinates to the single-nucleus sequencing data is available on
GitHub under the repository "broadchenf/Slide-tags".[3] This repository contains the necessary scripts to process the
spatial barcode library sequencing data and assign coordinates to the profiled nuclei.[3] The pipeline is designed to
work with demultiplexed FASTQ files from the spatial barcode library and gene expression data that has been processed
through standard tools like Cell Ranger.[3]
The Slide-tags technology, which the Trekker kit is based on, was developed at the Broad Institute of MIT and
Harvard.[4] The method involves tagging nuclei in intact tissue sections with spatial barcodes, which are then used to
reconstruct the original location of each nucleus after single-nucleus sequencing.[5][6][7] This approach allows for the
integration of spatial information with a variety of single-cell assays, including transcriptomics and
epigenomics.[6][8]
For those looking to analyze the data, Takara Bio offers a cloud-based analysis platform in collaboration with
LatchBio.[9][10] This platform provides a user-friendly interface for processing raw data and visualizing the results,
which may be beneficial for researchers with limited bioinformatics expertise.[1][10] A comprehensive user manual is
available to guide users through the analysis process on this platform.

## References

1. [takarabio.com](https://www.takarabio.com/learning-centers/spatial-omics/datasets-request)
2. [takarabio.com](https://www.takarabio.com/learning-centers/spatial-omics/trekker-resources/trekker-faqs)
3. [github.com](https://github.com/broadchenf/Slide-tags)
4. [curiobioscience.com](https://curiobioscience.com/press/curio-bioscience-launches-trekker-single-cell-spatial-mapping-kit/)
5. [broadinstitute.org](https://www.broadinstitute.org/news/new-method-tags-cells-location-coordinates-single-cell-studies)
6. [bridgeinformatics.com](https://bridgeinformatics.com/breakthrough-high-resolution-spatial-multi-omics-slide-tags-unlock-single-cell-analysis/)
7. [cbirt.net](https://cbirt.net/broad-scientists-introduce-slide-tags-a-novel-method-for-mapping-cells-in-single-cell-studies/)
8. [nih.gov](https://pubmed.ncbi.nlm.nih.gov/38093010/)
9. [latch.bio](https://blog.latch.bio/p/a-sneak-peak-into-new-spatial-analysis)
10. [takarabio.com](https://www.takarabio.com/documents/User%20Manual/Cloud/Cloud%20Analysis%20of%20Seeker%20and%20Trekker%20Data%20User%20Manual.pdf)

# Trekker Mouse Kidney Example

## Input Data

The pipeline requires several input files:

*   **Raw Sequencing Reads (FASTQ files):** Specifically, `TrekkerU_RATAC_MouseKidney1_R1_001.fastq.gz` and `TrekkerU_RATAC_MouseKidney1_R2_001.fastq.gz` which contain the genetic and barcode information.

```
@VL00269:355:AAG5VW3M5:1:1101:18989:1000 1:N:0:CGTACTAN+TATGCAGT
AGTTATCACCGTGATTGTGCGGAGACAATTAAGCATATCTGCCTTTTTTTTTTTTTTTTTTTTTTTTTTCC
+
IIIIIIII9IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII9IIIIIIIIIIIII
@VL00269:355:AAG5VW3M5:1:1101:19178:1000 1:N:0:CGTACTAN+TATGCAGT
TCCAGTTGGTATGTGATTCGGAATAGACAGGCCATTGCACTACGCATTTTTTTTTTTTTTTTTTTTTTTCC
+
I--9999--I-9I9I99II-IIII9II-I9IIIIIIII-III-IIIIIIIIIII99-I-IIIIIIIII9-I
@VL00269:355:AAG5VW3M5:1:1101:19519:1000 1:N:0:CGTACTAN+TATGCAGT
...
```

  *   **Bead Barcode Information:** The `U0027_016_BeadBarcodes.txt` file, which contains the sequences of the spatial barcodes on the slide.

```
CAGGTGCCAAATAG  1942.0  594.4
CATTCCATCGTCTA  2338.6  1531.6
...
```
  *   **Cell Ranger Output:** The directory `TrekkerU_RATAC_MouseKidney1_scRNAseqOut` contains the results of initial single-cell analysis, including files that define the valid cell barcodes.

```
└── [4.0K May  7 00:36]  TrekkerU_RATAC_ExampleInput_MouseKidney1
    ├── [895M May  7 00:34]  TrekkerU_RATAC_MouseKidney1_R1_001.fastq.gz
    ├── [710M May  7 00:34]  TrekkerU_RATAC_MouseKidney1_R2_001.fastq.gz
    ├── [4.0K May  9 18:16]  TrekkerU_RATAC_MouseKidney1_scRNAseqOut
    │   ├── [4.0K May  9 18:30]  TrekkerU_RATAC_MouseKidney1_ATAC_Cell_by_Peak_MEX
    │   │   ├── [596M May  9 18:18]  ATAC_Fragments.bed.gz
GL456210.1      789     841     53432314        3
GL456210.1      4239    4266    6830190 6
GL456210.1      4239    4282    46951500        2
...
    │   │   ├── [625K May  9 18:18]  ATAC_Fragments.bed.gz.tbi
    │   │   ├── [ 39K May  7 00:53]  atac-barcodes.tsv.gz
13236
19637
19697
32480
...
    │   │   ├── [1.5M May  7 00:53]  atac-features.tsv.gz
GL456233.2:133919-134450        GL456233.2:133919-134450        Peaks
GL456233.2:144822-145037        GL456233.2:144822-145037        Peaks
GL456233.2:206157-206441        GL456233.2:206157-206441        Peaks
...
    │   │   └── [ 45M May  7 00:53]  atac-matrix.mtx.gz
%%MatrixMarket matrix coordinate integer general
142635 10494 14164239
94 1 2
237 1 2
...
    │   └── [4.0K May  9 18:43]  TrekkerU_RATAC_MouseKidney1_RSEC_MolsPerCell_MEX
    │       ├── [ 39K May  7 00:35]  barcodes.tsv.gz
13236
19637
19697
...
    │       ├── [148K May  7 00:35]  features.tsv.gz
0610005C13Rik   0610005C13Rik   Gene Expression
0610006L08Rik   0610006L08Rik   Gene Expression
0610009B22Rik   0610009B22Rik   Gene Expression
0610009E02Rik   0610009E02Rik   Gene Expression
...
    │       ├── [ 13M May  7 00:35]  matrix.mtx.gz
    │       └── [377K May  9 18:43]  translate.csv
13236,TGTGTTCGCTTCGGAATAGGACGTTCA
19637,TGTGTTCGCTTATCCGAGTGTTGATGA
...
    └── [ 24M May  7 00:33]  U0027_016_BeadBarcodes.txt

5 directories, 12 files
```

## Output of the Processing

```
└── [4.0K May 12 20:34]  trekker_TrekkerU_RATAC_MouseKidney1
    ├── [4.0K May  7 00:13]  misc
    │   ├── [287K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_barcodes_perPublicFxn.tsv
    │   ├── [  70 May  7 00:13]  TrekkerU_RATAC_MouseKidney1_mismatchfreq.csv
    │   ├── [ 125 May  7 00:13]  TrekkerU_RATAC_MouseKidney1_properreads_matched_to_spatial_whitelist.csv
    │   ├── [328K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_reads_perMatchedCB.txt
    │   ├── [ 80M May  7 00:13]  TrekkerU_RATAC_MouseKidney1_reads_perMatchedCB_SB.txt
    │   ├── [  70 May  7 00:13]  TrekkerU_RATAC_MouseKidney1_summary_position_conf.txt
    │   ├── [  65 May  7 00:13]  TrekkerU_RATAC_MouseKidney1_summary_position_conf_consolidated.txt
    │   ├── [4.0K May  7 00:13]  U0027_016
    │   │   ├── [ 22M May  7 00:13]  BeadBarcodes.txt
    │   │   └── [ 12M May  7 00:13]  BeadLocations.txt
    │   ├── [ 80M May  7 00:13]  df_whitelist_TrekkerU_RATAC_MouseKidney1.txt
    │   ├── [ 174 May  7 00:13]  matcher_summary_TrekkerU_RATAC_MouseKidney1.txt
    │   ├── [ 32M May  7 00:13]  matching_result_TrekkerU_RATAC_MouseKidney1.csv
    │   └── [ 16M May  7 00:13]  reads_per_SB_TrekkerU_RATAC_MouseKidney1.txt
    └── [4.0K May 12 20:02]  output
        ├── [ 10M May  7 00:13]  TrekkerU_RATAC_MouseKidney1_ConfPositioned_anndata_matched.h5ad
        ├── [347M May 12 20:28]  TrekkerU_RATAC_MouseKidney1_ConfPositioned_seurat_spatial.rds
        ├── [293K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_Location_ConfPositionedNuclei.csv
        ├── [ 32M May  7 00:13]  TrekkerU_RATAC_MouseKidney1_MoleculesPer_ConfPositionedNuclei.mtx
        ├── [ 12M May  7 00:13]  TrekkerU_RATAC_MouseKidney1_Trekker_Report.html
        ├── [158K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_barcodes_ConfPositionedNuclei.tsv
        ├── [209K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_genes_ConfPositionedNuclei.tsv
        ├── [2.3K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_summary_metrics.csv
        ├── [ 74K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_variable_features_clusters.csv
        ├── [ 11K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_variable_features_spatial_moransi.txt
        ├── [4.0K May  7 00:13]  cell_bc_plots
        │   ├── [4.0K May  7 00:13]  cells_0_coordinates_assigned
        │   │   ├── [451K May  7 00:13]  ACGGATGGTGCTGTGTAGGCTCGTTAC.jpeg
    ...
        │   │   └── [441K May  7 00:13]  TGGTTCCTCTTACAGCGTACGAACGCA.jpeg
        │   ├── [4.0K May  7 00:13]  cells_1_coordinates_assigned
        │   │   ├── [446K May  7 00:13]  AGCGGCCAGTTGGTTAATTATTGCGCA.jpeg
   ...
        │   │   └── [454K May  7 00:13]  TCCTGGATTACCATCATAAGCTTGGAC.jpeg
        │   ├── [4.0K May  7 00:13]  cells_2_coordinates_assigned
        │   │   ├── [449K May  7 00:13]  ACTCGACCTTTATCCTGTGATCTGCAT.jpeg
 ....
        │   │   └── [479K May  7 00:13]  TGTGTTCGCGCGAGCCTTATAGGTCTA.jpeg
        │   └── [4.0K May  7 00:13]  cells_3_coordinates_assigned
        │       ├── [467K May  7 00:13]  AGAGATGTTGATTCCAGTGTGCTTGCA.jpeg
  ...
        │       └── [458K May  7 00:13]  TGGTTGTCCGCTCCGCTGATCAACGTC.jpeg
        ├── [1.2M May  7 00:13]  coords_TrekkerU_RATAC_MouseKidney1.txt
        ├── [4.0K May  7 00:13]  intermediates
        │   ├── [449K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_Location.csv
        │   ├── [320K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_Location_PositionedNuclei.csv
        │   ├── [ 55M May  7 00:13]  TrekkerU_RATAC_MouseKidney1_MoleculesPer.mtx
        │   ├── [ 36M May  7 00:13]  TrekkerU_RATAC_MouseKidney1_MoleculesPer_PositionedNuclei.mtx
        │   ├── [ 11M May  7 00:13]  TrekkerU_RATAC_MouseKidney1_Positioned_anndata_matched.h5ad
        │   ├── [370M May 12 20:28]  TrekkerU_RATAC_MouseKidney1_Positioned_seurat_spatial.rds
        │   ├── [ 16M May  7 00:13]  TrekkerU_RATAC_MouseKidney1_anndata_matched.h5ad
        │   ├── [286K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_barcodes.tsv
        │   ├── [173K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_barcodes_PositionedNuclei.tsv
        │   ├── [209K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_genes.tsv
        │   ├── [209K May  7 00:13]  TrekkerU_RATAC_MouseKidney1_genes_PositionedNuclei.tsv
        │   └── [533M May 12 20:27]  TrekkerU_RATAC_MouseKidney1_seurat_spatial.rds
        └── [4.0K May  7 00:13]  plots
            ├── [ 864 May  7 00:13]  summary_metrics.csv
            └── [ 254 May  7 00:13]  summary_minPts.csv

12 directories, 238 files

```

*   **Breakdown of the Output Directory:** The output is well-organized into distinct categories:

    *   **Final Processed Data (`output/`):** This is the most critical directory, containing the primary results ready for biological interpretation.
        *   **Spatial Coordinates:** The file `TrekkerU_RATAC_MouseKidney1_Location_ConfPositionedNuclei.csv` is a key output, providing the final calculated (x, y) coordinates for each confidently mapped cell barcode.
```
,SPATIAL_1,SPATIAL_2
TGTGTTCGCTCACACTTATCCATTCAT,4627.2793,-3301.5116
TGTGTTCGCTAGTTCAATGGTCCGACT,7811.34746,-6443.133325
...
```
        *   **Gene Expression Matrix:** The files `TrekkerU_RATAC_MouseKidney1_MoleculesPer_ConfPositionedNuclei.mtx`, with contents
```
%%MatrixMarket matrix coordinate integer general
27355 5785 2675218
187 1 1
544 1 1
1238 1 1
...
```
`...barcodes_ConfPositionedNuclei.tsv`, with contents
```
TGTGTTCGCTCACACTTATCCATTCAT
TGTGTTCGCTAGTTCAATGGTCCGACT
...
```


and `...genes_ConfPositionedNuclei.tsv`
```
0610005C13Rik
0610006L08Rik
0610009B22Rik
0610009E02Rik
...
```
together form the filtered gene expression count matrix, containing only the cells that were successfully placed in space.

        *   **Integrated Analysis Objects:** The pipeline conveniently provides the data in standard formats used by popular single-cell analysis toolkits:
            *   `...ConfPositioned_seurat_spatial.rds`: An R object for the Seurat package, with gene expression and spatial data integrated.
            *   `...ConfPositioned_anndata_matched.h5ad`: A Python AnnData object for use with packages like Scanpy and Squidpy.
        *   **HTML Report:** The `...Trekker_Report.html` file is a user-friendly summary of the entire analysis, containing key metrics and visualizations of the results.

    *   **Intermediate Files and Quality Control (`misc/`):** This directory contains the outputs from the intermediate steps of the pipeline, which are crucial for quality control and troubleshooting.
        *   `matching_result_...csv`: Shows the mapping between the spatial barcodes observed in the sequencing data and the official barcodes on the bead array.
        *   `df_whitelist_...txt`: A table that links the cell barcodes to the spatial barcodes they are associated with.
        *   `matcher_summary_...txt`: Provides high-level statistics on the barcode matching efficiency.

    *   **Visual Quality Control (`output/cell_bc_plots/`):** This directory provides direct visual evidence of the spatial positioning process.
        *   It contains JPEG images for individual cells, showing the spatial distribution of the barcode reads associated with them.
        *   The subdirectories (`cells_1_coordinates_assigned`, `cells_2_coordinates_assigned`, etc.) categorize cells by the number of distinct spatial clusters found, which helps in assessing the quality of the spatial localization. A cell with one clear cluster is ideally positioned.

    *   **Summary Metrics and Plots (`output/` and `output/plots/`):**
        *   The `...summary_metrics.csv` file provides a quantitative summary of the DBSCAN clustering performance, including the number of cells mapped and signal-to-noise ratios.
        *   The `plots` subdirectory contains figures summarizing the optimization of the DBSCAN clustering parameters (e.g., `summary_minPts.csv`).

*   **Final Output:** The primary output is a table that contains the unique identifier for each cell nucleus and its corresponding spatial coordinates in the tissue. This allows for the visualization and analysis of gene expression in a spatial context. The pipeline also generates various quality control plots and summary statistics at each stage.


Of course. Here is a revised and more detailed explanation of the processing workflow, focusing on the specific points you raised.

***

### The Source Code in `broadchenf/Slide-tags`

```
├── [1.3K]  README.md
├── [6.3K]  bead_matching.py
├── [7.7K]  cell_barcode_matcher.R
├── [2.2K]  sb_processing.sh
└── [ 33K]  spatial_positioning.R

1 directory, 5 files
```

*   **Processing Workflow:** The pipeline consists of four main steps, executed by a combination of shell, R, and Python scripts:
    1.  **FASTQ Processing (`sb_processing.sh`):** This initial step filters the raw sequencing data to find reads that contain the spatial barcode sequence and then downsamples the data to a manageable size.
        *   **Compute-Intensive Operations:** While the script's commands (`zgrep`, `zcat`, `awk`) appear simple, its computationally intensive nature comes from the sheer volume of the input data. The raw FASTQ files (`R1_path` and `R2_path`) are typically many gigabytes in size, containing hundreds of millions of sequencing reads. The script's most demanding task is the first `zgrep` command, which must decompress and scan through the entire multi-gigabyte `R2` file to find every instance of a specific 18-base DNA sequence ("TCTTCAGCGTTCCCGAGA"). Subsequently, the `awk` commands must process these enormous files again to extract the relevant reads. These operations are primarily limited by I/O (the speed at which data can be read from the disk) and CPU (for decompression and pattern matching).
        *   **Downsampling with `seqtk`:** `seqtk` is a fast and lightweight toolkit for processing sequencing files in FASTA or FASTQ format, and it is a standard tool in bioinformatics. In this script, `seqtk sample` is used for **downsampling**, which means to randomly select a smaller, representative subset of reads from a larger pool. The command `seqtk sample ... ${reads}` (where `${reads}` is 25,000,000) reduces the filtered data to 25 million reads. This is done because spatial barcode libraries are often sequenced to very high depths, generating more data than is necessary for accurate positioning. Downsampling creates a more manageable dataset for the subsequent R and Python scripts, significantly reducing their runtime and memory requirements without compromising the ability to identify the correct spatial location.

    2.  **Matching Spatial and Cell Barcodes (`cell_barcode_matcher.R`):** This script takes the processed sequencing reads and matches the spatial barcodes to the cell barcodes identified by the Cell Ranger software. It uses a whitelist of known 10x Genomics cell barcodes (`3M-february-2018.txt`) to ensure accuracy.

    3.  **Assigning Coordinates to Spatial Barcodes (`bead_matching.py`):** This script matches the spatial barcode sequences from the sequencing data to the known spatial barcode coordinates from the bead map. It employs both exact and "fuzzy" matching (allowing for small differences, or a low "edit distance") to account for potential sequencing errors.

    4.  **Spatial Mapping of Nuclei (`spatial_positioning.R`):** The final and most critical step uses a clustering algorithm to assign a precise (x, y) coordinate to each nucleus.
        *   **Clustering with DBSCAN:** The script uses **DBSCAN** (Density-Based Spatial Clustering of Applications with Noise), an algorithm well-suited for this task. The process works as follows for each individual cell:
            1.  **Create a Point Cloud:** For a single cell, the script gathers the (x, y) coordinates of all the unique spatial barcodes that were found associated with it. This forms a 2D point cloud.
            2.  **Identify the Core Cluster:** In an ideal experiment, all of these spatial barcodes should have come from a very small, dense area on the slide where the nucleus was located. DBSCAN excels at finding this dense region of points, grouping them into a primary cluster.
            3.  **Filter Noise:** The algorithm simultaneously identifies points that are far from this dense cluster as "noise" (labeled as cluster `0`). These are assumed to be the result of experimental artifacts and are excluded from the position calculation.
            4.  **Calculate the Weighted Centroid:** Once the primary cluster of spatial barcodes is identified, the script does not simply take the average coordinate. Instead, it calculates a **weighted centroid**. The "weight" for each spatial barcode coordinate is its UMI count (i.e., how many times that barcode was sequenced for that cell). This ensures that barcodes detected more frequently have a stronger influence on the final calculated position, making the result more robust and accurate.
            5.  **Parameter Optimization:** The script iterates through a range of `minPts` values (a key DBSCAN parameter) and selects the value that maximizes the number of cells that can be confidently mapped (i.e., cells that yield a single, unambiguous spatial cluster). This automated optimization makes the pipeline adaptive to datasets of varying quality.