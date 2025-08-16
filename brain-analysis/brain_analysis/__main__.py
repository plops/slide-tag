import anndata as ad
import sys

def main():
    """
    Loads an AnnData file and prints its observation columns.
    """
    # Define the path to the data file
    file_path = "/home/kiel/Downloads/trekker/Mouse_Brain_TrekkerR_merged_Aug2025/Mouse_Brain_TrekkerR_ConfPositioned_anndata_merged.h5ad"

    try:
        # Load the AnnData file
        print(f"Loading data from: {file_path}")
        adata = ad.read_h5ad(file_path)

        # The observation metadata is stored in the .obs attribute,
        # which is a pandas DataFrame.
        print("\nSuccessfully loaded the AnnData object.")
        print("AnnData object summary:")
        print(adata)

        # Print the column names from the .obs DataFrame
        print("\nColumns in adata.obs:")
        for col in adata.obs.columns:
            print(f"- {col}")

    except FileNotFoundError:
        print(f"Error: The file was not found at '{file_path}'", file=sys.stderr)
        print("Please make sure the H5AD file is in the correct directory.", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"An unexpected error occurred: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    main()