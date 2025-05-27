# Application Flow

Flow diagram link: https://excalidraw.com/#json=f_-QzgBmrKDlFvv3HvaGj,JhVQOirkO3RCxaY1KGIl0w

| Access type      | Optional        | Abilities
| :--              | :--:            | --:
| Owner (Server)   | No                                                        | Anything
| Admin            | Yes (if server-side adminstration is enabled)             | Anything except delete DB, reset server and edit configuration
| Reviewer         | Yes (if server-side adminstration is enabled)             | confirm changes and deny them
| Editor           | Yes                                                       | Send change requests from delete, add and edit
