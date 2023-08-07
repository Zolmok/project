use clap::Parser;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

type CmdWithArgs = (String, Vec<String>);

#[derive(Parser, Debug)]
#[clap(name = "Project")]
#[clap(author = "Ricky Nelson <rickyn@zolmok.org")]
#[clap(version = "0.4.0")]
#[clap(about = "Bootstrap a new project directory", long_about = None)]
struct Cli {
    #[clap(long, value_parser)]
    project_path: String,
}

#[derive(Debug)]
struct FileSetting {
    name: &'static str,
    contents: &'static str,
}
struct BinaryFileSetting {
    name: &'static str,
    contents: &'static [u8],
}

const ANDROID_CHROME_192: &'static [u8] = include_bytes!("./favicon/android-chrome-192x192.png");
const ANDROID_CHROME_512: &'static [u8] = include_bytes!("./favicon/android-chrome-512x512.png");
const APPLE_TOUCH: &'static [u8] = include_bytes!("./favicon/apple-touch-icon.png");
const FAVICON_16: &'static [u8] = include_bytes!("./favicon/favicon-16x16.png");
const FAVICON_32: &'static [u8] = include_bytes!("./favicon/favicon-32x32.png");
const FAVICON: &'static [u8] = include_bytes!("./favicon/favicon.ico");
const WEBMANIFEST: &'static [u8] = include_bytes!("./favicon/site.webmanifest");

/// Run a list of apps and print out the command and it's arguments before running
///
/// # Arguments
///
/// * `apps` - A list of apps to run
fn run_apps(cmd_with_args: Vec<CmdWithArgs>) -> Result<(), Box<dyn std::error::Error>> {
    for (cmd, args) in cmd_with_args {
        println!("");
        println!("========================");
        println!("$ {} {:?}", cmd, args);
        println!("========================");

        let mut command = Command::new(&cmd);
        command.args(&args);

        let output = command.output().expect("Failed to execute command");

        if !output.status.success() {
            panic!("Command {} exited with {:?}", cmd, output.status.code());
        } else {
            if cmd == "mkdir" {
                let rust_project_path = Path::new(&args[0]);

                // change to the project path
                match env::set_current_dir(&rust_project_path) {
                    Ok(_result) => {
                        println!("Directory {} has been created", &args[0])
                    }
                    Err(error) => panic!(
                        "Error [{}] while trying to set project directory: {}",
                        error, &args[0]
                    ),
                };
            }
            continue;
        }
    }

    Ok(())
}

fn create_directory(path: &str) {
    match fs::create_dir_all(&path) {
        Ok(_result) => {
            println!("Directory {} has been created", &path)
        }
        Err(error) => panic!(
            "Error [{}] while trying to create directory: {}",
            error, &path
        ),
    };
}

fn main() {
    let args = Cli::parse();
    let project_path = &args.project_path;
    let public_path = format!("{}/public", project_path);
    let src_path = format!("{}/src", project_path);

    [public_path, src_path]
        .iter()
        .for_each(|path| create_directory(path));

    // change to the project path
    match env::set_current_dir(&project_path) {
        Ok(_result) => {
            println!("Project directory has been set to {}", &project_path)
        }
        Err(error) => panic!(
            "Error [{}] while trying to set project directory: {}",
            error, &project_path
        ),
    };

    let git_init = (String::from("git"), vec!["init".to_string()]);
    let npm_init = (
        String::from("npm".to_string()),
        vec!["init".to_string(), "-y".to_string()],
    );
    let npm_install = (
        String::from("npm".to_string()),
        vec![
            "install".to_string(),
            "react".to_string(),
            "react-dom".to_string(),
        ],
    );
    let npm_install_dev = (
        String::from("npm".to_string()),
        vec![
            "install".to_string(),
            "--save-dev".to_string(),
            "@types/jest".to_string(),
            "@types/react".to_string(),
            "@vitejs/plugin-react".to_string(),
            "eslint".to_string(),
            "eslint-plugin-jest".to_string(),
            "eslint-plugin-react".to_string(),
            "eslint-plugin-react-hooks".to_string(),
            "jest".to_string(),
            "sass".to_string(),
            "vite".to_string(),
        ],
    );
    let package_json_update = r#"const fs = require('fs');

const packageJson = './package.json';
const contents = require(packageJson);

contents.scripts = {
  'dev:watch': 'vite',
  linter: 'eslint .',
  test: 'jest .',
};
contents.author = 'Ricky Nelson <rickyn@zolmok.org>';
contents.license = 'UNLICENSED';

fs.writeFile(packageJson, JSON.stringify(contents, null, 2), (err) => {
  if (err) {
    console.error(err);
  }
});"#;
    let update_package_json = (
        String::from("node".to_string()),
        vec!["-e".to_string(), package_json_update.to_string()],
    );

    let apps: Vec<CmdWithArgs> = vec![
        git_init,
        npm_init,
        npm_install_dev,
        npm_install,
        update_package_json,
    ];

    run_apps(apps).expect("One or more commands failed to execute");

    // create a .gitignore
    let git_ignore = FileSetting {
        name: ".gitignore",
        contents: r#"# web
node_modules

# rust
target
"#,
    };
    let prettier = FileSetting {
        name: "prettier.config.js",
        contents: r#"// https://prettier.io/docs/en/options.html
/** @type {import('prettier').RequiredOptions} */
module.exports = {
  trailingComma: 'es5',
  bracketSameLine: true,
  bracketSpacing: true,
  tabWidth: 2,
  semi: true,
  singleQuote: true,
  arrowParens: 'always',
  overrides: [
    {
      files: 'Routes.*',
      options: {
        printWidth: 999,
      },
    },
  ],
};
"#,
    };
    let eslintrc = FileSetting {
        name: ".eslintrc.json",
        contents: r#"{
  "env": {
    "browser": true,
    "es2021": true,
    "node": true
  },
  "extends": [
    "eslint:recommended",
    "plugin:jest/recommended",
    "plugin:react/recommended",
    "plugin:react-hooks/recommended"
  ],
  "overrides": [
    { "files": ["*.jsx", "*.js"] }
  ],
  "parserOptions": {
    "ecmaFeatures": {
      "jsx": true
    },
    "ecmaVersion": "latest",
    "sourceType": "module"
  },
  "plugins": [
    "jest",
    "react",
    "react-hooks"
  ],
  "rules": {
    "object-curly-spacing": ["error", "always"],
    "quotes": ["error", "single"],
    "react-hooks/rules-of-hooks": "error",
    "react-hooks/exhaustive-deps": "warn",
    "react/react-in-jsx-scope": "off",
    "react/jsx-uses-react": "off",
    "space-infix-ops": ["error", { "int32Hint": false }]
  },
  "settings": {
    "react": {
      "version": "detect"
    }
  }
}
"#,
    };
    let editorconfig = FileSetting {
        name: ".editorconfig",
        contents: r#"# EditorConfig is awesome: https://EditorConfig.org

# top-most EditorConfig file
root = true

# Unix-style newlines with a newline ending every file
[*]
end_of_line = lf
insert_final_newline = true
indent_style = space
indent_size = 2

[*.rs]
indent_size = 4

# Set default charset
charset = utf-8
"#,
    };
    let jsconfig = FileSetting {
        name: "jsconfig.json",
        contents: r#"{
  "compilerOptions": {
    "allowSyntheticDefaultImports": true,
    "baseUrl": "./src",
    "checkJs": true,
    "lib": ["dom", "es2017"],
    "jsx": "react-jsx",
    "moduleResolution": "node",
    "noEmit": true,
    "resolve": {
      "extensions": [".js", ".jsx", ".json"],
      "modules": ["src", "node_modules"]
    },
    "resolveJsonModule": true,
    "target": "esnext",
  },
  "exclude": ["node_modules", "dist"],
  "include": ["./**/*"]
}"#,
    };
    let html = FileSetting {
        name: "index.html",
        contents: r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <meta name="description" content="" />
    <meta
      name="viewport"
      content="width=device-width, initial-scale=1, shrink-to-fit=no"
    />

    <title>Project</title>

    <link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png">
    <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png">
    <link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png">
    <link rel="manifest" href="/site.webmanifest">

    <script src="src/app.jsx" type="module"></script>
  </head>
  <body>
    <div id="app"></div>
  </body>
</html>
"#,
    };
    let javascript = FileSetting {
        name: "index.js",
        contents: "console.log('hello, world!')",
    };
    let vite_config = FileSetting {
        name: "vite.config.js",
        contents: r#"import react from '@vitejs/plugin-react';
import { resolve } from 'path';
import { defineConfig } from 'vite';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      api: resolve(__dirname, './src/api'),
      components: resolve(__dirname, './src/components'),
      hooks: resolve(__dirname, './src/hooks'),
      layouts: resolve(__dirname, './src/layouts'),
      pages: resolve(__dirname, './src/pages'),
      utils: resolve(__dirname, './src/utils'),
    },
  },
});
"#,
    };

    let project_jsx = FileSetting {
        name: "src/project.jsx",
        contents: r#"export default function Project() {
  return <div>Test project ready to go!</div>;
}
"#,
    };
    let app_jsx = FileSetting {
        name: "src/app.jsx",
        contents: r#"import { createRoot } from 'react-dom/client';
import Project from './project';

const root = createRoot(document.getElementById('app'));

root.render(<Project />);
"#,
    };

    println!("");
    [
        git_ignore,
        prettier,
        eslintrc,
        editorconfig,
        jsconfig,
        html,
        javascript,
        vite_config,
        project_jsx,
        app_jsx,
    ]
    .iter()
    .for_each(|file| {
        println!("Creating file: {}", file.name);

        fs::write(file.name, file.contents).expect("Unable to write file");
    });

    let android_chrome_192 = BinaryFileSetting {
        name: "android-chrome-192x192.png",
        contents: ANDROID_CHROME_192,
    };
    let android_chrome_512 = BinaryFileSetting {
        name: "android-chrome-512x512.png",
        contents: ANDROID_CHROME_512,
    };
    let apple_touch = BinaryFileSetting {
        name: "apple-touch-icon.png",
        contents: APPLE_TOUCH,
    };
    let favicon_16 = BinaryFileSetting {
        name: "favicon-16x16.png",
        contents: FAVICON_16,
    };
    let favicon_32 = BinaryFileSetting {
        name: "favicon-32x32.png",
        contents: FAVICON_32,
    };
    let favicon = BinaryFileSetting {
        name: "favicon.ico",
        contents: FAVICON,
    };
    let webmanifest = BinaryFileSetting {
        name: "site.webmanifest",
        contents: WEBMANIFEST,
    };
    [
        android_chrome_192,
        android_chrome_512,
        apple_touch,
        favicon_16,
        favicon_32,
        favicon,
        webmanifest,
    ]
    .iter()
    .for_each(|file| {
        println!("Creating file: {}", file.name);

        fs::write(format!("./public/{}", file.name), file.contents).expect("Unable to write file");
    });

    println!("Done!");
}
