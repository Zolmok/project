use clap::Parser;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

type CmdWithArgs = (String, Vec<String>);

#[derive(Parser, Debug)]
#[clap(name = "Project")]
#[clap(author = "Ricky Nelson <rickyn@zolmok.org")]
#[clap(version = "0.1.0")]
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

fn main() {
    let args = Cli::parse();
    let project_path = &args.project_path;

    let create_directory = (String::from("mkdir"), vec![project_path.to_string()]);
    let git_init = (String::from("git"), vec!["init".to_string()]);
    let npm_init = (
        String::from("npm".to_string()),
        vec!["init".to_string(), "-y".to_string()],
    );
    let npm_install_vite = (
        String::from("npm".to_string()),
        vec![
            "install".to_string(),
            "--save-dev".to_string(),
            "vite".to_string(),
            "@vitejs/plugin-react".to_string(),
            "jest".to_string(),
            "eslint".to_string(),
            "eslint-plugin-jest".to_string(),
            "eslint-plugin-react".to_string(),
            "eslint-plugin-react-hooks".to_string(),
        ],
    );
    let npm_install_react = (
        String::from("npm".to_string()),
        vec!["install".to_string(), "react".to_string()],
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
        create_directory,
        git_init,
        npm_init,
        npm_install_vite,
        npm_install_react,
        update_package_json,
    ];

    run_apps(apps).expect("One or more commands failed to execute");

    // create a .gitignore
    let git_ignore = FileSetting {
        name: ".gitignore",
        contents: r#"node_modules
"#,
    };
    let prettier = FileSetting {
        name: ".prettierrc",
        contents: r#"{
  "bracketSpacing": false,
  "singleQuote": true,
  "printWidth": 80,
  "trailingComma": "all"
}
"#,
    };
    let eslintrc = FileSetting {
        name: ".eslintrc.json",
        contents: r#"{
  "env": {
    "browser": true,
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
    "baseUrl": ".",
    "target": "esnext",
    "allowSyntheticDefaultImports": true,
    "checkJs": true,
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

    <script src="index.js" type="module"></script>
  </head>
  <body>
    Test project ready to go!
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
    ]
    .iter()
    .for_each(|file| {
        println!("Creating file: {}", file.name);

        fs::write(file.name, file.contents).expect("Unable to write file");
    });
}
