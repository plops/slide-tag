import anndata as ad
import scanpy as sc
import numpy as np
import sys
import os
import matplotlib

matplotlib.use('Qt5Agg')  # Qt5Agg TkAgg
import matplotlib.pyplot as plt
from scipy.spatial import Voronoi, voronoi_plot_2d

# enable interactive plotting
plt.ion()

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

# load nucleus positions
ps0 = adata.obsm[
    'X_spatial']  # array([[ 4756.8087    , -6268.35166667], [ 8550.105     , -7147.409     ], ... ]]) shape=(31209, 2)

# rotate points by -100 degrees around the point 6700,-5380

# Center of rotation
cx, cy = 6700, -5380
theta = np.deg2rad(-127)  # Convert degrees to radians

# Rotation matrix
R = np.array([
    [np.cos(theta), -np.sin(theta)],
    [np.sin(theta), np.cos(theta)]
])

# Translate, rotate, and translate back
ps_centered = ps0 - np.array([cx, cy])
ps_rotated = ps_centered @ R.T
ps = ps_rotated + np.array([cx, cy])

# reduce size so that markers don't overlap
# plt.scatter(ps[:, 0], ps[:, 1], s=2, alpha=.8)

# find and set plot limits using 1 and 99 percentile of the points
# Calculate 10th and 90th percentiles for x and y
lo_perc = 1
hi_perc = 99
x_min, x_max = np.percentile(ps[:, 0], [lo_perc, hi_perc])
y_min, y_max = np.percentile(ps[:, 1], [lo_perc, hi_perc])

# Expand ranges by 10%
x_range = x_max - x_min
y_range = y_max - y_min

range_inc = .2 / 2
x_min_exp = x_min - range_inc * x_range
x_max_exp = x_max + range_inc * x_range
y_min_exp = y_min - range_inc * y_range
y_max_exp = y_max + range_inc * y_range

# the plot is a lot of white with some dots now
# i want to simulate cells. use delaunay (or something like that) to fill the canvas
vo = Voronoi(ps)


def polygon_area(poly):
    x, y = zip(*poly)
    return 0.5 * np.abs(np.dot(x, np.roll(y, 1)) - np.dot(y, np.roll(x, 1)))


# create a histogram of polygon areas in voronoi (bins and both scales logarithmic)

areas = []
for j in range(len(ps)):
    region = vo.regions[vo.point_region[j]]
    if not -1 in region and len(region) > 0:
        polygon = [vo.vertices[i] for i in region]
        area = polygon_area(polygon)
        areas.append(area)

# plt.figure()
# bins = np.logspace(np.log10(min(areas)), np.log10(max(areas)), 200)
# plt.hist(areas, bins=bins, log=True)
# plt.xlabel("Polygon Area")
# plt.xscale('log')
# plt.ylabel("Count")
# plt.title("Histogram of Voronoi Polygon Areas")

# area>3e4 seems to be a weird group (the periphery, ony two voronoi cells inside are bigger)


# voronoi_plot_2d(vo, show_points=False, show_vertices=False)
point_cluster = adata.obs['seurat_clusters'].values.astype('int')  # array([1, 2, 1, ..., 5 ... ) len=31209
cluster_color = adata.uns['seurat_clusters_colors']  # ['#1f77b4', '#ff7f0e'... ] len=19

for j in range(len(ps)):
    region = vo.regions[vo.point_region[j]]
    if not -1 in region:
        polygon = [vo.vertices[i] for i in region]
        if polygon_area(polygon) <= 3e4:
            plt.fill(*zip(*polygon), cluster_color[point_cluster[j]])

plt.xlim(x_min_exp, x_max_exp)
plt.ylim(y_min_exp, y_max_exp)
#  enforce equal scale on x and y axis
plt.gca().set_aspect('equal', adjustable='box')


# show the pre-processed umap visualization
umapfig = plt.figure(1,(17,8))
q = sc.pl.umap(
    adata,
    color="seurat_clusters",
    title="UMAP of Mouse Brain Cells by Cluster",
    frameon=False,
    show=False,  # This will display the plot interactively
)
plt.legend()

