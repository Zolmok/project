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
const RED_BOX_48: &'static [u8] = include_bytes!("./red-box-48.png");

/// Executes a sequence of shell commands with their respective arguments.
/// For each command, it prints a separator, the command being run, and then executes it.
/// If any of the commands exit with an error code, the function panics with a descriptive error message.
///
/// Special handling exists for the `mkdir` command; it changes the current working directory to
/// the newly created directory.
///
/// # Arguments
///
/// * `cmd_with_args` - A vector of tuples, where each tuple contains a command as a `String`
///   and its arguments as a `Vec<String>`.
///
/// # Returns
///
/// Returns `Ok(())` if all commands execute successfully.
/// Returns `Err` wrapped in a `Box<dyn std::error::Error>` if there's an error running any command.
///
/// # Panics
///
/// Panics if there's a failure in command execution or if changing the current directory fails.
///
/// # Examples
///
/// ```
/// run_apps(vec![
///     ("ls".to_string(), vec!["-l".to_string()]),
///     ("mkdir".to_string(), vec!["new_directory".to_string()]),
/// ]);
/// ```
///
/// Expected output: Details of commands being run and their outputs. Panics on errors.
fn run_apps(cmd_with_args: Vec<CmdWithArgs>) -> Result<(), Box<dyn std::error::Error>> {
    for (cmd, args) in cmd_with_args {
        // Print separator and command details
        println!("");
        println!("========================");
        println!("$ {} {:?}", cmd, args);
        println!("========================");

        // Initialize and set up the command
        let mut command = Command::new(&cmd);
        command.args(&args);

        // Run the command
        let output = command.output().expect("Failed to execute command");

        // Check the exit status
        if !output.status.success() {
            panic!("Command {} exited with {:?}", cmd, output.status.code());
        } else {
            // Special case for 'mkdir' command
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

/// Creates a new directory at the specified `path`. If the parent directories
/// do not exist, it creates them as well. If the directory creation is successful,
/// it prints a confirmation message. In case of any error during directory
/// creation, the function panics with a descriptive error message.
///
/// # Arguments
///
/// * `path` - A string slice representing the path where the directory should be created.
///
/// # Panics
///
/// Panics if there's an error during directory creation, displaying the specific error and path.
///
/// # Examples
///
/// ```
/// create_directory("/path/to/directory");
/// ```
///
/// Expected output: `Directory /path/to/directory has been created`
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

// The main function to set up a new project.
fn main() {
    // Parsing command-line arguments.
    let args = Cli::parse();

    // Constructing paths for the project.
    let project_path = &args.project_path;
    let public_path = format!("{}/public", project_path);
    let images_path = format!("{}/public/images", project_path);
    let src_path = format!("{}/src", project_path);

    // Creating necessary directories in the project.
    [public_path, images_path, src_path]
        .iter()
        .for_each(|path| create_directory(path));

    // Change the current directory to the project path.
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

    // Define commands to initialize git and npm in the project directory.
    // Each command is represented as a tuple with the command name and its arguments.
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
            "tailwindcss".to_string(),
            "postcss".to_string(),
            "autoprefixer".to_string(),
        ],
    );
    let package_json_update = r#"const fs = require('fs');

const packageJson = './package.json';
const contents = require(packageJson);

contents.scripts = {
  'build': 'vite build',
  'dev:watch': 'vite --open',
  linter: 'eslint .',
  test: 'jest .',
};
contents.author = 'Ricky Nelson <rickyn@zolmok.org>';
contents.license = 'UNLICENSED';
contents.type = 'module';

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

    // Run the defined commands.
    run_apps(apps).expect("One or more commands failed to execute");

    // Define file content settings for creating configuration files.
    // Each file is represented with its name and content.
    let git_ignore = FileSetting {
        name: ".gitignore",
        contents: r#"# web
node_modules
dist

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
  return (
    <main>
      <header className="relative isolate">
        <div className="mx-auto max-w-7xl px-4 py-10 sm:px-6 lg:px-8">
          <div className="mx-auto flex max-w-2xl items-center justify-between gap-x-8 lg:mx-0 lg:max-w-none">
            <div className="flex items-center gap-x-6">
              <img
                src="/images/red-box-48.png"
                alt="Project"
                className="h-16 w-16 flex-none"
              />
              <h1>
                <div className="mt-1 text-base font-semibold leading-6 text-gray-900">
                  Project
                </div>
              </h1>
            </div>
          </div>
        </div>
      </header>

      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div className="mx-auto grid max-w-2xl grid-cols-1 grid-rows-1 items-start gap-x-8 gap-y-8 lg:mx-0 lg:max-w-none lg:grid-cols-3">
          Test project ready to go!
        </div>
      </div>
    </main>
  );
}
"#,
    };
    let app_jsx = FileSetting {
        name: "src/app.jsx",
        contents: r#"import { createRoot } from 'react-dom/client';
import Project from './project';

import './index.css';

const root = createRoot(document.getElementById('app'));

root.render(<Project />);
"#,
    };
    let postcss_config = FileSetting {
        name: "postcss.config.js",
        contents: r#"export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
};
"#,
    };
    let index_css = FileSetting {
        name: "src/index.css",
        contents: r#"@tailwind base;
@tailwind components;
@tailwind utilities;
"#,
    };
    let tailwind_config = FileSetting {
        name: "tailwind.config.js",
        contents: r#"/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./index.html', './src/**/*.{js,ts,jsx,tsx}'],
  theme: {
    extend: {},
  },
  plugins: [],
};
"#,
    };

    // Write out all the configuration files.
    println!("");
    [
        git_ignore,
        prettier,
        eslintrc,
        editorconfig,
        jsconfig,
        html,
        vite_config,
        project_jsx,
        app_jsx,
        postcss_config,
        index_css,
        tailwind_config,
    ]
    .iter()
    .for_each(|file| {
        println!("Creating file: {}", file.name);

        fs::write(file.name, file.contents).expect("Unable to write file");
    });

    // Define binary file content settings for icons and images.
    // Each binary file is represented with its name and content.
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
    let red_box_48 = BinaryFileSetting {
        name: "red-box-48.png",
        contents: RED_BOX_48,
    };

    // Write out all the binary files.
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

    println!("Creating file: red-box-48.png");
    fs::write("./public/images/red-box-48.png", red_box_48.contents).expect("Unable to write file");

    println!("Done!");
}
