# VS Code vs Zed Comparison

## Architecture Comparison

| Component | VS Code Version | Zed Version |
|-----------|----------------|-------------|
| **Language** | TypeScript | Rust |
| **Runtime** | Node.js | Rust (WASM) |
| **Entry Point** | `src/extension.ts` | `src/lib.rs` |
| **API** | `vscode` module | `zed` module (planned) |
| **Build System** | `package.json` | `Cargo.toml` |

## Code Comparison

### Configuration Management

#### VS Code (TypeScript)
```typescript
// src/config.ts
import { workspace } from 'vscode';
export const CONFIG_SECTION = "zokuzoku";
export default () => workspace.getConfiguration(CONFIG_SECTION);
```

#### Zed (Rust)
```rust
// src/config.rs
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

pub struct ConfigManager {
    config: Config,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let mut config = Config::default();
        config.set_default("enabled", false)?;
        // ... more defaults
        Ok(Self { config })
    }
}
```

### IPC Communication

#### VS Code (TypeScript)
```typescript
// src/core/hachimiIpc.ts
import * as vscode from 'vscode';
import http from "node:http";

function call(command: Command): Promise<CommandResponse> {
    return new Promise((resolve, reject) => {
        const req = http.request({
            hostname: config().get("hachimiIpcAddress"),
            port: 50433,
            method: "POST",
            // ...
        });
    });
}
```

#### Zed (Rust)
```rust
// src/ipc.rs
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct HachimiIpc {
    client: Client,
    address: String,
    port: u16,
}

impl HachimiIpc {
    pub async fn call(&self, command: HachimiCommand) -> Result<HachimiResponse> {
        let response = self
            .client
            .post(&url)
            .json(&command)
            .send()
            .await?;
        // ...
    }
}
```

### File Operations

#### VS Code (TypeScript)
```typescript
// src/core/utils.ts
import * as vscode from 'vscode';
import fs from 'fs/promises';

export function expandEnvironmentVariables(path: string): string {
    // Expand %VAR% on Windows, $VAR on Unix
}
```

#### Zed (Rust)
```rust
// src/utils.rs
pub fn expand_environment_variables(path: &str) -> String {
    if cfg!(windows) {
        // Expand %VAR% syntax
    } else {
        // Expand $VAR or ${VAR} syntax
    }
}
```

### Python Integration

#### VS Code (TypeScript)
```typescript
// src/pythonBridge.ts
import { spawn } from 'child_process';

async function execute<T>(command: string, params: object): Promise<T> {
    const childProcess = spawn(pythonExecutable, [
        '-u',
        bridgeScriptPath,
        command,
        paramsJson
    ]);
    // ...
}
```

#### Zed (Rust)
```rust
// src/python_bridge.rs (planned)
use pyo3::prelude::*;

#[pyfunction]
fn extract_story_data(params: PyDict) -> PyResult<StoryData> {
    // Call UnityPy from Rust
    Python::with_gil(|py| {
        let unitypy = py.import("UnityPy")?;
        // ... extract story data
    })
}
```

## File Mapping

| VS Code File | Zed File | Status |
|-------------|----------|--------|
| `src/config.ts` | `src/config.rs` | ✅ Complete |
| `src/commands.ts` | `src/commands.rs` | ✅ Scaffold |
| `src/core/hachimiIpc.ts` | `src/ipc.rs` | ✅ Complete |
| `src/core/utils.ts` | `src/utils.rs` | ✅ Complete |
| `src/core/downloader.ts` | `src/downloader.rs` | ✅ Complete |
| `src/sqlite/index.ts` | `src/sqlite.rs` | ✅ Scaffold |
| `src/core/localizedDataManager.ts` | `src/data_manager.rs` | ✅ Scaffold |
| `src/core/jsonDocument.ts` | `src/json_document.rs` | ❌ Pending |
| `src/core/assetHelper.ts` | `src/asset_helper.rs` | ❌ Pending |
| `src/core/automation.ts` | `src/automation.rs` | ❌ Pending |
| `src/pythonBridge.ts` | `src/python_bridge.rs` | ❌ Pending |
| `src/pythonInterop.ts` | `src/python_interop.rs` | ❌ Pending |
| `src/editors/*.ts` | `src/editors/*.rs` | ❌ Pending |
| `src/views/*.ts` | `src/views/*.rs` | ❌ Pending |
| `webviews/src/**/*.svelte` | `src/ui/**/*.rs` | ❌ Pending |

## Performance Comparison

### Expected Improvements

| Operation | VS Code | Zed | Improvement |
|-----------|---------|-----|-------------|
| Extension startup | ~500ms | ~50ms | 10x faster |
| File operations | Node.js I/O | Native I/O | 2-3x faster |
| HTTP requests | Node.js stream | Native async | Comparable |
| Python calls | Spawn subprocess | Direct FFI | 5-10x faster |
| Memory usage | High (V8) | Low (native) | ~50% less |

## API Differences

### Tree Views

#### VS Code
```typescript
vscode.window.createTreeView('stories', {
    treeDataProvider: new StoriesTreeDataProvider()
});
```

#### Zed (Planned)
```rust
// Will use Zed's tree view API once available
let tree_view = zed::create_tree_view("stories", provider);
```

### Custom Editors

#### VS Code
```typescript
vscode.window.registerCustomEditorProvider(
    'zokuzoku.storyEditor',
    new StoryEditorProvider(context)
);
```

#### Zed (Planned)
```rust
// Will use Zed's custom editor API once available
zed::register_custom_editor("storyEditor", provider);
```

### Commands

#### VS Code
```typescript
vscode.commands.registerCommand('zokuzoku.enable', () => {
    // ...
});
```

#### Zed (Planned)
```rust
// Will use Zed's command API once available
zed::register_command("zokuzoku.enable", |_| {
    // ...
});
```

## Migration Challenges

### 1. Python Integration
- **VS Code**: Uses pymport (Node.js Python binding)
- **Zed**: Needs PyO3 implementation
- **Challenge**: Set up PyO3 and bridge UnityPy

### 2. UI Framework
- **VS Code**: Webviews with Svelte
- **Zed**: Native UI (API not yet finalized)
- **Challenge**: Need to rewrite all UI components

### 3. Extension API
- **VS Code**: Mature API with many features
- **Zed**: Evolving API with limited features
- **Challenge**: Wait for API maturity or work around limitations

### 4. Asset Parsing
- **VS Code**: Python scripts with UnityPy
- **Zed**: Need Python bridge or native implementation
- **Challenge**: Complex binary format parsing

## Benefits of Zed Version

1. **Performance**: Native Rust code is faster than Node.js
2. **Resource Usage**: Lower memory footprint
3. **Type Safety**: Rust's type system prevents many bugs
4. **Concurrency**: Better async/await implementation
5. **Packaging**: Single binary distribution

## Drawbacks of Zed Version

1. **Development Time**: Longer initial development
2. **API Availability**: Need to wait for Zed API
3. **Learning Curve**: Rust is more complex than TypeScript
4. **Python Integration**: PyO3 setup is complex
5. **UI**: Need to rewrite all UI components

## Conclusion

The Zed version is a significant rewrite that will:
- ✅ Improve performance
- ✅ Reduce resource usage
- ✅ Provide better type safety
- ❌ Take longer to develop
- ❌ Require new skills
- ❌ Need Zed API to mature

**Recommendation**: Keep VS Code version as primary, develop Zed version as experimental/proof-of-concept until Zed API matures.

