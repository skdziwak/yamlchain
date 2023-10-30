# YamlChain 
YamlChain is an innovative CLI application that leverages OpenAI's GPT models to generate text based on pre-defined prompts. Using declarative YAML workflows, this tool built in Rust provides supreme precision and control over the AI's actions. It can be seamlessly integrated into your development workflow for generating code, creating documentation, text processing, and much more.

## Installation
The installation process of the AI workflow tool is straightforward but depends on you having Rust installed in your machine. If you don't have Rust already set up, you can check the installation guide [here](https://www.rust-lang.org/tools/install).

Once you have Rust setup, installing the AI Workflow Tool involves the following steps:

1. Clone the git repository:
    ```bash
    git clone https://github.com/skdziwak/yamlchain.git
    ```
2. Move into the project directory:
    ```bash
    cd yamlchain
    ```
3. Compile and install the project:
    ```bash
    cargo install --path .
    ```

## Usage
To use YamlChain, you need to provide a YAML file which defines the workflow. This file specifies the steps that the tool should follow and can be loaded into the tool using the `-f` flag.

Here's a simple example:

```bash
yamlchain -f ./my-workflows.yaml
```

You also have the option to specify the particular workflow from the YAML file that you want to run by using the `name` parameter:

```bash
yamlchain -f ./my-workflows.yaml edit_files
```

It needs `OPENAI_API_KEY` environment variable which can be specified in `.env` file.

You can also use

```bash
yamlchain --help
```

```
Usage: yamlchain [OPTIONS] [workflow_name]

Arguments:
  [workflow_name]  Name of the workflow to run, if not specified and there is only one workflow in the file, it will be used

Options:
  -s, --schema <SCHEMA>
          Saves JSON schema for the workflows file
  -f, --workflows-file <WORKFLOWS_FILE>
          Path to the workflows file
  -i, --interface <INTERFACE>
          Interface to use, if not specified, vim interface will be used. [possible values: cli, vim]
  -w, --workdir <WORKDIR>
          Working directory for the workflow, if not specified, current directory will be used.
  -d, --debug
          Enable debug logs
  -h, --help
          Print help
  -V, --version
          Print version
```

## Example

This workflow starts by providing a description of the project. It then collects the user's goal or what they want to implement. Using this input, it generates user stories and provides the user an opportunity to review and finalize these stories. Next, it reads the project tree to identify relevant files. It also reads the content of these relevant files and then proceeds to write code for the new feature. The code is refined through a feedback loop and, finally, saved to a file.

```yaml
workflows:
  - name: create_stage
    description: This workflow allows you to implement new features in this project.
    stages:
      # Explains the purpose and structure of the project
      - name: description
        stage:
          type: set
          value: |
            This project is a declarative yaml-based tool for defining workflows.
            A workflow is made of stages, which include user input, processing by GPT models, feedback loops,
            automated testing, executing bash scripts etc.
            It consists of modules, each of those can control program flow, call OpenAI API, print output, save files etc.
            Those modules are called stages.

      # Gathers user input on what feature they want to implement
      - name: goal
        stage:
          type: user_input
          message: |
            Describe what you want to implement.

      # Generates user stories based on the user input
      - name: user-stories
        stage:
          type: ai_processing
          model: gpt-4
          system_message: |
            ${description}
            Your goal is to write user stories based on user input.
            User will tell you what feature he needs, and you will give him a list.
          prompt: |
            Here is what I want to implement:
            ${goal}

      # Allows the user to review the generated user stories
      - name: user-stories-fl
        stage:
          type: user_input
          message: |
            Those are generated user stories. Uncomment the ones you like.
            ${user-stories}

      # Reads the project tree from the src directory
      - name: project-tree
        stage:
          type: shell_command
          command: tree
          args:
            - "src"

      # Determines which files in the project are most relevant to the new feature
      - name: relevant-files
        stage:
          type: ai_processing
          model: gpt-4
          system_message: |
            ${description}
            You will be provided with a project tree and user stories for a new feature.
            Your task is to give me a list of relative paths of relevant files starting with ./src 
            Those files will be presented to the programmer, so he knows the context of the project.
            Maximum 5 files. Two of those files should be example stages from workflows/stages that might help you.
          prompt: |
            ${project-tree}
            User stories:
            ${user-stories-fl}

      # Transforms the output of the previous stage into a list
      - name: relevant-files-list
        stage:
          type: ai_reshape
          model: gpt-3.5-turbo
          target: list
          data: ${relevant-files}

      # Retrieves the content of the identified relevant files
      - name: relevant-files-content
        stage:
          type: shell_script
          script: |
            while IFS= read -r line; do
                [[ -z $line ]] && continue
                echo "$line"
                if [[ -f $line && -r $line ]]; then
                    cat "$line"
                else
                    echo "Error: Unable to read file $line"
                fi
                echo "========="
            done
          stdin: ${relevant-files-list}

      # Writes the code for the new feature
      - name: coding
        stage:
          type: ai_processing
          model: gpt-4
          system_message: |
            ${description}
            You will be provided with a list of relevant files and user stories for a new feature.
            Your task is to write code for this feature in just one file at workflows/stages path
            You only output the code, no comments, no file names, no markdown code blocks, just the code itself.
          prompt: |
            Relevant files:
            ${relevant-files-content}
            User stories:
            ${user-stories-fl}

      # Provides a feedback loop to refine the generated code
      - name: coding-fl
        stage:
          type: feedback_loop
          model: gpt-4
          info_message: |
            Relevant files:
              ${relevant-files-content}
            User stories:
              ${user-stories-fl}
            Your task is to improve the code.
          initial_input: ${coding}

      # Captures the name of the module from the user
      - name: module 
        stage:
          type: user_input
          message: What is the name of the module?

      # Saves the final code to a Rust source file
      - name: save
        stage:
          type: save_file
          path: src/workflows/stages/${module}.rs
          content: ${coding-fl}

      # Adds the newly created module to the main stages file
      - name: add-module
        stage:
          type: shell_script
          script: |
            echo "pub mod ${module};" >> src/workflows/stages.rs
```

## LICENSE

MIT License

Copyright (c) 2023 Szymon Dziwak <skdziwak@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## DISCLAIMER AND LIABILITY LIMITATION FOR PROJECT SAFETY

NOTICE: BY UTILIZING THIS PROJECT, YOU ACKNOWLEDGE THE INHERENT RISKS ASSOCIATED WITH ITS FUNCTIONALITIES AND AGREE TO THE TERMS SET FORTH IN THIS DISCLAIMER.

1. **Inherent Risks and Safety Concerns**: This project, by design, allows for the execution of shell commands, scripts (including but not limited to Python scripts), and other arbitrary operations. This capability entails significant security risks, including potential unintended or malicious operations.

2. **Potential Input from OpenAI API**: The project has the capacity to run scripts or commands provided directly by the OpenAI API or other external sources. The execution of such automatically generated scripts or commands without adequate scrutiny and validation poses severe security threats.

3. **Mandatory Review**: Prior to executing any workflow, especially those involving scripts, commands, or external API inputs, YOU MUST review the content diligently. Ensure that all operations, scripts, and commands are known, safe, and intended.

4. **No Liability**: The authors, contributors, and maintainers of this project shall bear no responsibility or liability for any damage, loss, or compromise arising from the use of this project. Your use of this project signifies your acceptance of this risk, and you agree to indemnify and hold harmless all parties associated with the project from any claims or actions arising from its use.

5. **Recommendation**: It's strongly recommended to run this project in isolated environments, devoid of sensitive data or systems, and always have the latest security patches and updates applied.

6. **No Warranty**: This project is provided "AS IS", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and non-infringement. In no event shall the authors, contributors, or copyright holders be liable for any claim, damages, or other liabilities, whether in an action of contract, tort, or otherwise, arising from, out of, or in connection with the project or the use or other dealings in the project.

By continuing to use or interact with this project, you signify your explicit acceptance of this disclaimer and all its terms. If you do not agree with any of these stipulations, you must refrain from using the project.

