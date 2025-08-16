# Accessing Takara Bio Trekker / Slide-tags Datasets and Source Code

Researchers interested in utilizing the Takara Bio Trekker / Slide-tags technology for spatially resolved single-cell analysis can access both example datasets and the underlying source code to facilitate their work.

## Example Datasets:

Takara Bio provides a collection of datasets generated with the Trekker Single-Cell Spatial Mapping Kit. These datasets span various human and mouse tissues, including mouse brain, embryo, and kidney, as well as human breast cancer and melanoma.[1] While most of these are output files, some raw sequencing data in FASTQ format are also available for download.[1] Access to these datasets requires registration on the Takara Bio website, after which login credentials will be provided via email.[1]
These example datasets allow researchers to explore the capabilities of the Trekker and its associated analysis pipelines without having to generate their own data initially.[1] The Trekker system is designed to be species-agnostic and compatible with various single-cell platforms like the 10x Genomics Chromium and BD Rhapsody systems.[1][2]

## Source Code:

The computational pipeline for assigning spatial coordinates to the single-nucleus sequencing data is available on GitHub under the repository "broadchenf/Slide-tags".[3] This repository contains the necessary scripts to process the spatial barcode library sequencing data and assign coordinates to the profiled nuclei.[3] The pipeline is designed to work with demultiplexed FASTQ files from the spatial barcode library and gene expression data that has been processed through standard tools like Cell Ranger.[3]
The Slide-tags technology, which the Trekker kit is based on, was developed at the Broad Institute of MIT and Harvard.[4] The method involves tagging nuclei in intact tissue sections with spatial barcodes, which are then used to reconstruct the original location of each nucleus after single-nucleus sequencing.[5][6][7] This approach allows for the integration of spatial information with a variety of single-cell assays, including transcriptomics and epigenomics.[6][8]
For those looking to analyze the data, Takara Bio offers a cloud-based analysis platform in collaboration with LatchBio.[9][10] This platform provides a user-friendly interface for processing raw data and visualizing the results, which may be beneficial for researchers with limited bioinformatics expertise.[1][10] A comprehensive user manual is available to guide users through the analysis process on this platform.

# References

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
