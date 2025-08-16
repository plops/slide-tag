import anndata as ad
import scanpy as sc
import sys
import os

# def main():
"""
Loads a spatial transcriptomics AnnData file, generates a spatial plot
of cell clusters, and saves it to a file.
"""
# Define paths
file_path = "/home/kiel/Downloads/trekker/Mouse_Brain_TrekkerR_merged_Aug2025/Mouse_Brain_TrekkerR_ConfPositioned_anndata_merged.h5ad"
output_dir = "plots"
output_filename = "mouse_brain_clusters_spatial.png"
output_path = os.path.join(output_dir, output_filename)

# Create the output directory if it doesn't exist
os.makedirs(output_dir, exist_ok=True)

try:
    # Load the AnnData file
    print(f"Loading data from: {file_path}")
    adata = ad.read_h5ad(file_path)

    # The spatial coordinates are in 'X_spatial', so we tell scanpy to use them.
    # Note: Scanpy by default looks for a key named 'spatial', but your object
    # has 'X_spatial'. We can simply rename it for compatibility.
    if 'X_spatial' in adata.obsm and 'spatial' not in adata.obsm:
        adata.obsm['spatial'] = adata.obsm['X_spatial']

    print("\nSuccessfully loaded the AnnData object.")
    print("AnnData object summary:")
    print(adata)

    # --- Plotting Section ---
    print(f"\nGenerating spatial plot, colored by 'seurat_clusters'...")

    # Use scanpy's spatial plotting function.
    # - We color the spots (cells) by the 'seurat_clusters' column.
    # - `spot_size` is adjusted to make the plot readable. You may need to
    #   tweak this value.
    # - `frameon=False` removes the plot frame for a cleaner look.
    # - `save=output_path` will save the figure directly to the specified file.
    sc.pl.spatial(
        adata,
        color="seurat_clusters",
        spot_size=0.03,
        frameon=False,
        show=False,  # Do not display interactively, just save
        save="_temp_plot.png"  # Scanpy adds prefixes, we'll rename
    )

    # Scanpy's save function can be quirky with naming.
    # We rename the saved file to our desired output path.
    # The default save name from sc.pl.spatial is often 'show_temp_plot.png'
    # or similar, let's find it and rename it.
    # A common default name is 'spatial.png' in the ./figures/ directory.
    default_save_dir = 'figures'
    default_filename = 'spatialseurat_clusters.png'
    default_saved_path = os.path.join(default_save_dir, default_filename)

    if os.path.exists(default_saved_path):
        os.rename(default_saved_path, output_path)
        # Clean up the directory scanpy creates
        if not os.listdir(default_save_dir):
            os.rmdir(default_save_dir)
        print(f"Plot successfully saved to: {output_path}")
    else:
        print(f"Warning: Could not find the plot file at '{default_saved_path}'. Please check the 'figures' directory.",
              file=sys.stderr)


except FileNotFoundError:
    print(f"Error: The file was not found at '{file_path}'", file=sys.stderr)
    sys.exit(1)
except KeyError as e:
    print(f"Error: A required key was not found in the AnnData object: {e}", file=sys.stderr)
    print("Please ensure 'X_spatial' and 'seurat_clusters' are present.", file=sys.stderr)
    sys.exit(1)
except Exception as e:
    print(f"An unexpected error occurred: {e}", file=sys.stderr)
    sys.exit(1)

# if __name__ == "__main__":
#     main()
