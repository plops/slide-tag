[ This was written with the help of AI ]

# Mouse Brain Spatial Analysis

This project uses Python to load and analyze spatial transcriptomics data from a mouse brain experiment.

## Setup

This project uses `uv` for package and environment management.

1. **Install uv:**
   Follow the instructions
   at [https://docs.astral.sh/uv/getting-started/installation/](https://docs.astral.sh/uv/getting-started/installation/)

2. **Create the virtual environment and install dependencies:**
   ```sh
   uv sync
   ```

## Running the Script

To load the data and print its columns, run:

```sh
uv run brain_analysis
```

## Description

After loading the anndata file we see:

```python
>> > adata
AnnData
object
with n_obs × n_vars = 31209 × 30105
obs: 'nCount_RNA', 'nFeature_RNA', 'percent.mt', 'log10_nCount_RNA', 'log10_nFeature_RNA', 'nCount_SCT', 'nFeature_SCT', 'SCT_snn_res.0.2', 'seurat_clusters'
var: 'name'
uns: 'seurat_clusters_colors'
obsm: 'X_pca', 'X_spatial', 'X_umap', 'spatial'
```

```
AnnData object with n_obs × n_vars = 31209 × 30105
```

This is the most important line. It tells you the dimensions of your main gene expression matrix.

* **`n_obs` (Observations): 31,209**
  In single-cell analysis, an "observation" is a single cell (or in this case, a single **nucleus**). So, your dataset
  contains the genetic information for 31,209 individual nuclei.

* **`n_vars` (Variables): 30,105**
  A "variable" is a feature that is measured for each observation. In transcriptomics, this means a **gene**. So, your
  dataset contains the expression levels of 30,105 different genes.
*

Essentially, the core of your `adata` object is a massive table with 31,209 rows (nuclei) and 30,105 columns (genes).

---

### `obs`: Observation Metadata

The `.obs` attribute is a table (a pandas DataFrame) containing metadata *about each nucleus*.

```
>>> adata.obs
                                                    nCount_RNA  nFeature_RNA  percent.mt  log10_nCount_RNA  log10_nFeature_RNA  nCount_SCT  nFeature_SCT SCT_snn_res.0.2 seurat_clusters
Mouse_Brain_LaneA_TrekkerR_seurat_TGTGTTCGCTTCT...      2291.0          1071   27.891750          3.360025            3.029789      2463.0          1071               1               1
Mouse_Brain_LaneA_TrekkerR_seurat_TGTGTTCGCTTCC...      3903.0          2024    0.384320          3.591399            3.306211      3366.0          2022               2               2
...
[31209 rows x 9 columns]
```

It has 31,209 rows, one for each nucleus, and the columns listed are attributes of those nuclei.

* `nCount_RNA`: The total number of RNA molecules (UMIs) detected in that nucleus. It's a measure of library size or
  sequencing depth per cell.
* `nFeature_RNA`: The number of unique genes detected in that nucleus. It's a measure of genetic complexity.
* `percent.mt`: The percentage of RNA molecules that came from mitochondrial genes. For single-**nucleus** data, this is
  an important quality control metric. A high percentage can indicate a damaged nucleus where cytoplasmic contents (
  which contain mitochondria) have contaminated the sample.
* `log10_...`: Log-transformed versions of the count and feature metrics, which are often used for visualization and
  statistical modeling.
* `nCount_SCT` / `nFeature_SCT`: Counts and features after applying a specific normalization method called "SCTransform"
  from the Seurat package. This suggests the data was processed in R with Seurat before being loaded into Python. See
  the appendix for a description of the SCTransform.
* `SCT_snn_res.0.2` / `seurat_clusters`: **These are your cell clusters.** Based on gene expression similarity, the
  nuclei have been grouped together. `seurat_clusters` is likely the primary result, and these clusters serve as our
  best guess for different cell types (e.g., cluster '0' might be one type of neuron, cluster '1' another, etc.). This
  is the column you are using to color your plot.

---

### `var`: Variable Metadata

```
>>> adata.var
                        name
0610005C13Rik  0610005C13Rik
0610006L08Rik  0610006L08Rik
0610009B22Rik  0610009B22Rik
0610009E02Rik  0610009E02Rik
0610009L18Rik  0610009L18Rik
...                      ...
Xlr5c                  Xlr5c
Zcchc13              Zcchc13
Zdhhc25              Zdhhc25
Zfp683                Zfp683
Zfp969                Zfp969

[30105 rows x 1 columns]
```

The `.var` attribute is a DataFrame containing metadata *about each gene*. It has 30,105 rows, one for each gene. In
this case, it's very simple and just contains one column, `name`, which holds the official names of the genes (e.g., '
Gad1', 'Slc17a7').

---

### `uns`: Unstructured Metadata

```
>>> adata.uns
OrderedDict({'seurat_clusters_colors': ['#1f77b4', '#ff7f0e', '#279e68', '#d62728', '#aa40fc', '#8c564b', '#e377c2', '#b5bd61', '#17becf', '#aec7e8', '#ffbb78', '#98df8a', '#ff9896', '#c5b0d5', '#c49c94', '#f7b6d2', '#dbdb8d', '#9edae5', '#ad494a']})
```

The `.uns` attribute (for "unstructured") is a dictionary for storing dataset-level information that doesn't fit into
the `obs` or `var` tables.

* `seurat_clusters_colors`: This is very convenient. It stores the specific color codes that have been assigned to each
  of the clusters in `obs['seurat_clusters']`. When you tell Scanpy to plot by `seurat_clusters`, it automatically finds
  these colors and uses them, ensuring your plots are consistent.

---

### `obsm`: Multi-dimensional Observation Data

```
>>> adata.obsm
AxisArrays with keys: X_pca, X_spatial, X_umap, spatial
>>> adata.obsm['X_pca']
array([[ 1.02934457e+01,  3.62081661e+00, -1.05177368e+01, ...
>>> adata.obsm['X_spatial']
array([[ 4756.8087    , -6268.35166667], ...
>>> adata.obsm['X_umap']
array([[-2.80912987, -0.52719992], ...
>>> adata.obsm['spatial']
array([[ 4756.8087    , -6268.35166667], ...
```

The `.obsm` attribute stores matrix-like data where the rows still correspond to the nuclei, but the columns are
something other than genes. This is primarily used for dimensionality reductions and coordinates.

* `X_pca`: The results of Principal Component Analysis. It's a lower-dimensional (e.g., 50 dimensions) representation of
  the original 30,105-gene expression data.
* `X_umap`: The results of the UMAP algorithm, which is a technique to visualize the high-dimensional PCA data in a 2D
  plot. This is used to create plots where similar cells appear close together in abstract space.
* `X_spatial`: **This is your key spatial data.** It's a table with 31,209 rows and 2 columns, representing the (x,
  y) coordinates for each nucleus in the brain tissue.
* `spatial`: This is a copy of `X_spatial` that your script created. The Scanpy plotting function `sc.pl.spatial`
  specifically looks for a key named `'spatial'`, so creating this copy ensures compatibility.

### In a Nutshell

Your `adata` object is a self-contained "package" for your entire experiment. It holds:

1. The core gene expression data (`n_obs` x `n_vars`).
2. The cell type identities (`obs['seurat_clusters']`).
3. The physical location of each cell (`obsm['spatial']`).

This is why you can generate the final, colored spatial diagram with a single command: you are telling the plotting
function to use the coordinates from `obsm['spatial']` and apply colors based on the groups defined in
`obs['seurat_clusters']`.

# Appendix

## On the `SCTransform` (single cell transform)

Biological heterogeneity in single-cell RNA-seq data is often confounded by technical factors
including sequencing depth. The number of molecules detected in each cell can vary significantly between cells, even
within the same celltype. Interpretation of scRNA-seq data requires effective pre-processing and normalization to remove
this technical variability. The key idea behind SCTransform is to use a regularized negative binomial regression model
to account for the technical noise [1]. This modeling framework stabilizes the variance of the data, making it more
suitable for downstream analyses like dimensionality reduction, clustering, and differential expression.

### The Core Problem: Separating Signal from Noise

In any experiment, the goal is to separate the true physical or biological signal from technical noise. In single-cell
RNA sequencing:

* **The Signal:** The biologically meaningful variation in gene expression between different cell types or states. This
  is what you want to study.
* **The Noise:** Technical variation introduced during the experiment. The most significant source of this noise is *
  *sequencing depth** (the total number of RNA molecules captured per cell [FIXME: Is that correct?]). A cell that was
  sequenced more deeply will
  show higher counts for *all* its genes, which can easily be mistaken for a biological effect.

The purpose of SCTransform is to build a precise mathematical model of the technical noise so that it can be cleanly
removed, leaving just the biological signal.

Here is what each part of the name means in that context:

---

### 1. Regression Model

This is the fundamental approach. Regression is used to model the relationship between variables. Here, we build a model
for each gene that aims to **predict its expression count based solely on a technical factor**—the cell's total
sequencing depth.

* **Independent Variable (Input):** Total sequencing depth of a cell (`nCount_RNA`).
* **Dependent Variable (Output):** The observed count for a single gene in that cell.

The resulting model for each gene represents the **expected technical baseline**. Any deviation from this baseline is
considered a biological signal. The difference between the actual observed count and the model's prediction is called a
**residual**. These residuals are the new, "normalized" values that SCTransform uses for downstream analysis.

### 2. Negative Binomial Distribution

This part answers the question: "What kind of mathematical function should our regression model use?" The choice of
function must match the statistical properties of the data.

Gene expression data consists of counts, so a first thought might be to use a **Poisson distribution**. A Poisson
process describes the probability of a given number of events occurring in a fixed interval if these events occur with a
known constant mean rate and independently of the time since the last event. A key property of the Poisson distribution
is that its **variance is equal to its mean**.

However, single-cell data has two main sources of variance:

1. **Shot Noise (Technical):** The random sampling process of capturing and sequencing molecules. This is well-described
   by a Poisson distribution.
2. **Biological Noise (Signal):** Genes are not expressed at a perfectly constant rate. They are transcribed in "
   bursts," leading to extra variability in the number of RNA molecules present in a cell at any given moment.

This second source of variance means that the total variance in the data is **greater than the mean**. This phenomenon
is called **overdispersion**.

The **Negative Binomial** distribution is a more flexible alternative to the Poisson. It is also a distribution for
count data, but it has an additional parameter that explicitly models this overdispersion. It can account for both
sources of variance simultaneously, making it a much more accurate model for the noisy, "bursty" nature of gene
expression.

### 3. Regularization

This is a standard technique in machine learning and statistics to prevent **overfitting**.

Here's the problem: you are fitting a separate model for *every single gene* in your dataset (often 20,000+ models). For
genes that are expressed at very low levels, the data is extremely sparse (mostly zeros). Trying to fit a complex model
to such sparse data is unreliable; the model might latch onto random noise and produce unstable or nonsensical parameter
estimates. This is overfitting.

**Regularization** solves this by adding a constraint or penalty to the model-fitting process. This penalty discourages
the model's parameters from taking on extreme values. In practice, the algorithm enforces a "prior belief" that the
relationship between gene abundance and the noise parameters should be smooth and well-behaved across all genes. It *
*pools information across genes with similar expression levels** to get more robust and stable parameter estimates for
every individual gene, especially the sparse ones.

---

### Putting It All Together

So, a **"regularized negative binomial regression model"** is:

A **regression model** that predicts a gene's count from a cell's sequencing depth, using a **negative binomial
distribution** to accurately capture the data's mean-variance relationship (overdispersion), and employing *
*regularization** to ensure the model parameters are stable and robust, even for genes with very little data.

The final output is a set of residuals (the difference between measured and predicted values) where the technical noise
has been effectively modeled and removed.

# References

1. SCTransform Paper https://genomebiology.biomedcentral.com/articles/10.1186/s13059-021-02584-9