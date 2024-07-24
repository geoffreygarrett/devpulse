import os
import requests

GITHUB_API_URL = "https://api.github.com"
REPO = os.getenv("GITHUB_REPOSITORY")
RUN_ID = os.getenv("GITHUB_RUN_ID")
TOKEN = os.getenv("GITHUB_TOKEN")

def get_annotations(repo, run_id, token):
    headers = {
        "Authorization": f"token {token}",
        "Accept": "application/vnd.github.v3+json",
    }
    url = f"{GITHUB_API_URL}/repos/{repo}/actions/runs/{run_id}/jobs"
    response = requests.get(url, headers=headers)
    response.raise_for_status()
    jobs = response.json()["jobs"]

    annotations = []
    for job in jobs:
        job_id = job["id"]
        url = f"{GITHUB_API_URL}/repos/{repo}/actions/jobs/{job_id}/annotations"
        response = requests.get(url, headers=headers)
        response.raise_for_status()
        annotations.extend(response.json()["annotations"])

    return annotations

def main():
    if not all([REPO, RUN_ID, TOKEN]):
        print("Missing required environment variables.")
        exit(1)

    annotations = get_annotations(REPO, RUN_ID, TOKEN)

    if annotations:
        print("Annotations found:")
        for annotation in annotations:
            print(annotation)
    else:
        print("No annotations found.")
        exit(1)

if __name__ == "__main__":
    main()
