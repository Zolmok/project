use clap::Parser;
use std::env;
use std::fs;
use std::path::Path;

use scuttle::{App, Args};

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
fn run_apps(apps: &[App]) {
    for app in apps.iter() {
        println!("");
        println!("========================");
        println!("$ {} {}", app.command, Args(app.args.to_owned()));
        println!("========================");

        match scuttle::run_status(app) {
            Err(error) => panic!("panic{}", error),
            Ok(_status) => {
                if app.command == "mkdir" {
                    let rust_project_path = Path::new(&app.args[0]);

                    // change to the project path
                    match env::set_current_dir(&rust_project_path) {
                        Ok(_result) => {
                            println!("Directory {} has been created", &app.args[0])
                        }
                        Err(error) => panic!(
                            "Error [{}] while trying to set project directory: {}",
                            error, &app.args[0]
                        ),
                    };
                }
                continue;
            }
        };
    }
}

fn main() {
    let args = Cli::parse();
    let project_path = &args.project_path;

    let create_directory = scuttle::App {
        command: String::from("mkdir"),
        args: vec![project_path.to_string()],
    };
    let git_init = scuttle::App {
        command: String::from("git"),
        args: vec!["init".to_string()],
    };
    let npm_init = scuttle::App {
        command: String::from("npm".to_string()),
        args: vec!["init".to_string(), "-y".to_string()],
    };
    let npm_install_parcel = scuttle::App {
        command: String::from("npm".to_string()),
        args: vec![
            "install".to_string(),
            "--save-dev".to_string(),
            "parcel".to_string(),
            "jest".to_string(),
            "eslint".to_string(),
            "eslint-plugin-jest".to_string(),
            "eslint-plugin-react".to_string(),
            "eslint-plugin-react-hooks".to_string(),
        ],
    };
    let package_json_update = r#"const fs = require('fs');

const packageJson = './package.json';
const contents = require(packageJson);

contents.scripts = {
  'dev:watch': 'parcel index.html',
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
    let update_package_json = scuttle::App {
        command: String::from("node".to_string()),
        // the best way to update a JSON file is with JavaScript
        args: vec!["-e".to_string(), package_json_update.to_string()],
    };

    let apps: &[App] = &[
        create_directory,
        git_init,
        npm_init,
        npm_install_parcel,
        update_package_json,
    ];

    run_apps(apps);

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

    <title></title>

    <script src="index.js" type="module"></script>
  </head>
  <body></body>
</html>
"#,
    };
    let javascript = FileSetting {
        name: "index.js",
        contents: "console.log('hello, world!')",
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
    ]
    .iter()
    .for_each(|file| {
        println!("Creating file: {}", file.name);

        fs::write(file.name, file.contents).expect("Unable to write file");
    });
}
