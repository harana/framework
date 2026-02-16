# Harana FML — VS Code Extension

Syntax highlighting for the **Harana Framework Modelling Language** (`.fml`) — a YAML-based DSL used to define actions, objects, events, and web objects in the Harana framework.

## Features

- **Distinct highlighting for `action` and `object`** keywords (and `event` / `webobject`) so entity types are immediately distinguishable
- Full syntax highlighting for:
  - **Entity declarations**: `action`, `object`, `event`, `webobject`
  - **Section keywords**: `inputs`, `outputs`, `schema`, `attributes`, `events`, `flow`, `steps`, …
  - **Type system**: `string`, `int`, `float`, `boolean`, `date`, `id`, `any`, `list[…]`, `map[…]`
  - **Annotations / constraints**: `#email`, `#unique`, `#max(255)`, `#min(3)`, `#url`, `#json`
  - **Enum values**: `admin | user | guest`
  - **Default values**: `= value`
  - **Optional markers**: `?`
  - **Foreign-key references**: `-> user.id`
  - **Template expressions**: `{{ variable }}`
  - **Comments**: `# comment`
- Bundled **dark** and **light** colour themes optimised for FML
- Bracket auto-closing, indentation rules, and code folding

## Colour Scheme (Dark Theme)

| Token | Colour | Example |
|-------|--------|---------|
| `action` keyword | Red `#FF7B72` **bold** | `- action: build` |
| `object` keyword | Purple `#D2A8FF` **bold** | `- object: user` |
| `event` keyword | Orange `#FFA657` **bold** | `- event: app_started` |
| `webobject` keyword | Blue `#79C0FF` **bold** | `- webobject: button` |
| Action name | Orange | `build`, `push`, `pull` |
| Object/type name | Blue | `user`, `geo_location` |
| Section keywords | Red | `inputs:`, `outputs:`, `schema:` |
| Types | Blue italic | `string`, `int`, `list[…]` |
| Keys | Green | `email:`, `username:` |
| Annotations | Grey italic | `#email`, `#max(255)` |

## Installation

### From source (local)

```bash
cd vscode/fml
npm install
npm run package        # produces harana-fml.vsix
npm run install-local  # packages + installs into VS Code
```

### Manual VSIX install

```bash
code --install-extension harana-fml.vsix
```

## File Association

The extension automatically activates for any file with the `.fml` extension. To use it with existing `.yml` files, add this to your VS Code settings:

```json
{
  "files.associations": {
    "*.fml": "fml"
  }
}
```

## Build Scripts

| Script | Description |
|--------|-------------|
| `npm run package` | Package the extension into a `.vsix` file |
| `npm run publish` | Publish to the VS Code Marketplace |
| `npm run install-local` | Package and install locally |
| `make package` | Same as `npm run package` (via Makefile) |
| `make install` | Same as `npm run install-local` (via Makefile) |
| `make clean` | Remove build artefacts |

## Development

1. Open the `vscode/fml` folder in VS Code
2. Press **F5** to launch an Extension Development Host
3. Open any `.fml` file to see syntax highlighting in action

## License

MIT
