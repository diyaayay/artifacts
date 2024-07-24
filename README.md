## GitHub Artifacts - Storing workflow data as artifacts
GitHub Artifacts allow you to store and share data generated during a GitHub Actions workflow. Artifacts can be any files or directories, such as build outputs, test results, or logs, that you want to persist and use in other jobs within the same workflow or download later.
They allow us to persist data after a job has completed, and share that data with another job in the same workflow.

These are some of the common artifacts that we can upload:

- Log files and core dumps
- Test results, failures, and screenshots
- Binary or compressed files
- Stress test performance output and code coverage results

## Use Cases
- **Build Outputs**: Save compiled binaries or packages for further testing or deployment.
- **Test Results**: Store test logs or coverage reports for later analysis.
- **Logs**: Preserve logs from different stages of the workflow for debugging.

## Differences from other similar techniques
- **Artifacts**: Temporary storage, usually for intermediate files used within the workflow or for a short period after the workflow completes. Artifacts can be accessed and downloaded by anyone with the appropriate permissions.
- **Releases**: Permanent storage, used for versioning and distributing final, stable versions of software. Releases are often used for publishing software binaries, changelogs, and other release-related information.
- **Dependency Caching**: Artifacts and caching are similar because they provide the ability to store files on GitHub, but each feature offers different use cases and cannot be used interchangeably.
> [!NOTE]  
> Use caching when you want to reuse files that don't change often between jobs or workflow runs, such as build dependencies from a package management system. Use artifacts when you want to save files produced by a job to view after a workflow run has ended, such as built binaries or build logs.

## How to Set Up GitHub Actions Artifacts

1. **Create a Workflow File**
   - Create a YAML file in the `.github/workflows` directory of your repository. For example, `.github/workflows/artifact-demo.yml`.

2. **Define Jobs and Steps**
   - Set up jobs and steps where you will create and upload artifacts.

3. **Upload Artifacts**
   - Use the `actions/upload-artifact` action to upload files or directories as artifacts.

4. **Download Artifacts**
   - Use the `actions/download-artifact` action to download artifacts in subsequent jobs.

### Example Workflow

```yaml
name: Build and Store Artifact - A Demo for pr05 skill share session

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v2

      - name: Set up Rust
        uses: actions/setup-rust@v1
        with:
          rust-version: 'stable'

      - name: Build the program
        run: cargo build --release

      - name: Upload build artifact
        uses: actions/upload-artifact@v3
        with:
          name: Diya Rust Program Binary ;)
          path: target/release/diya_artifact

```

## Things to keep in mind
1. **Keep Artifacts Small**: Only store necessary files to avoid hitting storage limits.
2. **Clean Up**: Regularly clean up old artifacts if they are no longer needed.
3. **Security**: Be mindful of sensitive information. Avoid storing secrets or credentials as artifacts.