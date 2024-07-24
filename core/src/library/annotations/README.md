# Annotations Library

This library provides a unified interface for creating annotations across multiple CI/CD platforms, including GitHub
Actions, Azure DevOps, GitLab CI/CD, and Bitbucket Pipelines.

## Directory Structure

```plaintext
lib
└── annotations
    ├── README.md
    ├── annotation.rs
    ├── annotator.rs
    ├── azure
    │   ├── azure_annotator.rs
    │   └── mod.rs
    ├── github
    │   ├── github_annotator.rs
    │   └── mod.rs
    ├── gitlab
    │   ├── gitlab_annotator.rs
    │   └── mod.rs
    ├── bitbucket
    │   ├── bitbucket_annotator.rs
    │   └── mod.rs
    └── service.rs
```

## Structure

- `annotation.rs`: Contains the data structures for annotations.
- `annotator.rs`: Defines the trait for issuing annotations.
- `service.rs`: Provides a service to get the appropriate annotator based on the platform.
- `azure/`: Contains Azure-specific annotation logic.
- `github/`: Contains GitHub-specific annotation logic.
- `gitlab/`: Contains GitLab-specific annotation logic.
- `bitbucket/`: Contains Bitbucket-specific annotation logic.

## Usage

To use this library, instantiate the appropriate annotator based on your CI/CD platform and issue annotations.

## Annotation Capabilities Comparison

| Feature                          | GitHub Actions                        | Azure DevOps                 | GitLab CI/CD       | Bitbucket Pipelines          |
|----------------------------------|---------------------------------------|------------------------------|--------------------|------------------------------|
| **Annotation Types**             | `error`, `warning`, `notice`, `debug` | `error`, `warning`, `notice` | `error`, `warning` | `error`, `warning`, `notice` |
| **File & Line Specific**         | Yes                                   | Yes                          | Yes                | Yes                          |
| **Column Specific**              | Yes                                   | Yes                          | Yes                | No                           |
| **Grouping Log Lines**           | Yes                                   | No                           | Yes                | No                           |
| **Environment Variable Masking** | Yes                                   | Yes                          | Yes                | Yes                          |
| **Job Summary**                  | Yes                                   | No                           | No                 | No                           |
| **Add System Path**              | Yes                                   | Yes                          | Yes                | Yes                          |
| **Custom Message Titles**        | Yes                                   | Yes                          | Yes                | No                           |
| **Conditional Annotations**      | Yes                                   | Yes                          | Yes                | Yes                          |

### Notes

- **GitHub Actions**: Provides extensive annotation capabilities including custom titles, column-specific annotations,
  and detailed job summaries.
- **Azure DevOps**: Offers robust annotation features but lacks grouping log lines and job summary capabilities.
- **GitLab CI/CD**: Supports basic annotation types and includes column-specific annotations, but does not have job
  summaries or grouped log lines.
- **Bitbucket Pipelines**: Supports basic annotation types and is more limited in customization options compared to
  other platforms. Column-specific annotations are not supported.

### Example Annotations

#### GitHub Actions

```bash
echo "::error file=app.js,line=1,col=5,endColumn=7::Missing semicolon"
echo "::notice file=app.js,line=1::Consider refactoring this function"
```

#### Azure DevOps

```bash
echo "##vso[task.logissue type=error;sourcepath=app.js;linenumber=1;columnnumber=5;endcolumnnumber=7;]Missing semicolon"
echo "##vso[task.logissue type=warning;sourcepath=app.js;linenumber=1;]Consider refactoring this function"
```

#### GitLab CI/CD

```yaml
script:
  - echo "Error in file:
      app.js on line:
        1, column: 5. Missing semicolon"
  - echo "Warning: Consider refactoring this function in app.js on line 1"
```

#### Bitbucket Pipelines

```yaml
script:
  - echo "Annotation:
      error:
        Missing semicolon in file:
          app.js on line: 1"
  - echo "Annotation:
      warning:
        Consider refactoring this function in file: app.js on line 1"
```

This table and notes should help in understanding the differences in annotation capabilities across these platforms and
how to implement them.
