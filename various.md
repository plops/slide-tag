The primary difference between these spatial biology approaches lies in the biological molecules they target and their specific imaging mechanisms. MERFISH is a high-plex spatial transcriptomics method for mapping RNA, while SUM-PAINT (Secondary Label-Based Unlimited Multiplexed DNA-PAINT) is a super-resolution proteomics technique for resolving proteins at single-molecule resolution. [1, 2, 3, 4] 
Comparison of MERFISH and SUM-PAINT

| Feature [1, 3, 4, 5, 6, 7, 8, 9, 10] | MERFISH | SUM-PAINT |
|---|---|---|
| Primary Target | RNA (Transcriptomics) | Protein (Proteomics) |
| Technology Base | Single-molecule FISH (smFISH) | DNA-PAINT (Super-resolution) |
| Resolution | Subcellular (diffraction-limited) | Single-protein resolution |
| Mechanism | Combinatorial labeling & error-robust barcoding | Secondary label-based sequential DNA-PAINT |
| Multiplexing | Hundreds to thousands of genes | Virtually unlimited protein species |
| Commercial Status | Available via Vizgen MERSCOPE | Research protocol (see STAR Protocols[](https://pubmed.ncbi.nlm.nih.gov/40048420/)) |

Key Differences

* MERFISH (Multiplexed Error-Robust Fluorescence In Situ Hybridization):
* How it works: It uses sequential rounds of hybridization with barcoded probes to identify specific RNA transcripts. Its "error-robust" nature comes from binary barcodes that can detect and correct misread bits during imaging.
   * Best for: Mapping cell types, states, and neighborhood interactions across large tissue sections (up to $3\text{ cm}^2$) with high transcript sensitivity.
* SUM-PAINT (Secondary Label-Based Unlimited Multiplexed DNA-PAINT):
* How it works: This is a super-resolution microscopy technique. It relies on the transient binding of short, fluorescently labeled DNA "imager" strands to complementary "docking" strands attached to antibodies. By using different DNA sequences for different proteins, it can image an "unlimited" number of targets sequentially.
   * Best for: Ultra-high resolution protein mapping, such as resolving synaptic heterogeneity in neuronal cell atlases at the single-protein level. [1, 2, 3, 4, 6, 8] 

Are you planning to map RNA transcripts across a whole tissue or do you need super-resolution protein localization for specific cellular structures?

[1] [https://pubmed.ncbi.nlm.nih.gov](https://pubmed.ncbi.nlm.nih.gov/40048420/)
[2] [https://www.youtube.com](https://www.youtube.com/watch?v=lJMK6udLFHs&t=42)
[3] [https://www.youtube.com](https://www.youtube.com/watch?v=Ni8Vefi7P8Q&t=124)
[4] [https://pmc.ncbi.nlm.nih.gov](https://pmc.ncbi.nlm.nih.gov/articles/PMC11928808/)
[5] [https://advanced.onlinelibrary.wiley.com](https://advanced.onlinelibrary.wiley.com/doi/10.1002/advs.202520806?af=R)
[6] [https://www.biocompare.com](https://www.biocompare.com/Editorial-Articles/622786-Advancing-Spatial-Biology-with-Fluorescence-Based-Multiplexed-Imaging/)
[7] [https://elifesciences.org](https://elifesciences.org/reviewed-preprints/96949.pdf#:~:text=While%20both%20types%20of%20approaches%20exhibit%20distinct,principle%2C%20unambiguously%20assign%20molecules%20to%20single%20cells.)
[8] [https://vizgen.com](https://vizgen.com/wp-content/uploads/2026/01/MERFISH-2.0-Flyer_Digital.pdf)
[9] [https://vizgen.com](https://vizgen.com/study-published-in-nature-neuroscience-spotlights-the-use-of-vizgens-merscope-platform-to-uncover-new-insights-about-aging/)
[10] [https://www.cell.com](https://www.cell.com/star-protocols/fulltext/S2666-1667%2825%2900043-7#:~:text=Figures%20were%20created%20with%20the%20help%20of%20BioRender%20%28https://biorender.com%29.)
