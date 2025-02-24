**Linear Regression Model**
# Introduction
This project implements a simple linear regression model using the burn library in Rust. 
The goal is to generate synthetic data, define a linear regression model, train the model on the synthetic data, and evaluate its performance. 
The project demonstrates the use of machine learning techniques in Rust and provides insights into the challenges and solutions encountered during the implementation.

## Approach
1) **Define Synthetic Data*
   It is to generate a dataset of (x, y) pairs where y=2x+1 with some added noise to simulate real-world conditions.
   This is done using the rand crate.
   The synthetic data helps in training the model to learn the underlying relationship between x and y.
   
2) **Define Model*
   Then define a simple linear regression model using the burn library.
   The model consists of a weight and a bias, both initialized randomly.
   The forward pass computes the predicted values, and the loss function is defined using Mean Squared Error (MSE).
   
3) **Train Model*
   The model is trained using the Stochastic Gradient Descent (SGD) optimizer.
   We iterate over the dataset for a specified number of epochs, updating the model parameters to minimize the loss.
   The training process is monitored to ensure the model converges.
4) **Evaluate Model*
   After training, we evaluate the model on unseen data and plot the results using the textplots crate.
## Results and Evaluation
The model is to be trained on synthetic data and evaluated on unseen data. The results was meant to show 
that the model is able to learn the underlying relationship between x and y, with the predictions closely matching the actual values. 
The plot generated using the textplots crate should visually demonstrates the model's performance.

## Reflection on Learning Process
1) **Challenges**
   A) **Import Errors*: Initially, I encountered unresolved import errors for burn::optim::sgd::Sgd, burn::optim::OptimizerConfig, and burn::loss::mean_squared_error.
   I was able to resolve these by research into using the correct import paths and ensuring the burn library version matched the specified dependencies. 
   B) **Generic Arguments*: I faced issues with the Tensor type requiring generic arguments. This was resolved by specifying the type and backend for Tensor.
   C) **Declaration and parameter*: I struggled with alot of declaration that did not match the function or generic numbers of parameters, which I failed to resolve upon investigation.
2) **Experience**
   As a first-time Rust programmer, this project has been a valuable learning experience.
   While there were moments of frustration and difficulty, each challenge provided an opportunity to gaun knowledge of Rust and its ecosystem.
   The process of debugging, consulting documentation, and iterating on the code has strengthened my problem-solving skills and resilience.
   Even though I faced obstacles and the project did not run from the second to the fourth step, the journey itself has been rewarding.
   It has laid a strong foundation for future Rust projects and instilled confidence in tackling more complex problems.
   The experience has also underscored the importance of continuous learning and the willingness to seek help from various resources.
   
4) **Resources**
a) **AI Tools*: Used AI tools to assist with code generation and debugging.
b) **Library Documentation*: Referred to the burn library documentation for correct usage and import paths.
c) **Tutorials*: Followed Rust tutorials and examples for implementing linear regression and using the burn library.
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
# Setting Up Rust Project and Rust Rover IDE
This section provides the steps to set up a Rust project and configure it in Rust Rover IDE, including setting up Git integration with GitHub. 
It covers the installation of Rust, setting up Rust Rover IDE, creating a new Rust project, adding the necessary dependencies, and connecting the project to GitHub.
## Prerequisites
 - Before setting up the project, make sure you have a working internet connection.
**Install Rust**
- use the official website to download rust: [Rust Installation Page](https://www.rust-lang.org/tools/install).
- Run the exe to install
- Verify installation by open your terminal or command prompt.
- Run the following command to verify that Rust is installed correctly:
     ```bash
     rustc --version
     ```
- You should see the Rust version installed on your system if the installation was successful.
- If not, try and restart your PC and run the above command.
- 
**Download and Install Rust Rover IDE**
- use the official website to download Rust Rover IDE: [Rust Rover Download](https://www.jetbrains.com/rust/).
- Download the version suitable for your operating system (Windows, macOS, or Linux).
- Visual Studio Code and IntelliJ will be installed as part of Rust Rover
- Make sure Rust version is enabled on the Plugins and added on the Language and Frameworks on your IDE settings

## STEP 1: Create a New Rust Project

1. **Create the Project Using Rust Rover**:
   - Open Rust Rover IDE.
   - From the main menu, click `File` > `New Project`.
   - Choose `Rust` as the project type.
   - Select the location where you want the project to be saved and give it a name (e.g., `linear_regression_model`).
   - Click `Create` to generate the new project.

2. **Alternatively, Create the Project from the Command Line**:
   - Open your terminal and navigate to the folder where you want to create the project.
   - Run the following commands:
     ```bash
     cargo new linear_regression_model
     cd linear_regression_model
     ```
   - Open the newly created project folder in Rust Rover by going to `File` > `Open` and selecting the `linear_regression_model` folder.

## STEP 2:Configure Cargo.toml

1. **Edit Cargo.toml**:
   - Inside the project folder, locate the `Cargo.toml` file.
   - Replace its content with the following TOML configuration under the `[dependencies]` section:
     ```toml
     [dependencies]
     burn = { version = "0.16.0", features = ["wgpu", "train"] }
     burn-ndarray = "0.16.0"
     rand = "0.9.0"
     rgb = "0.8.50"
     textplots = "0.8.6"
     ```
   - Make sure to copy the dependencies exactly as shown above. Any changes that do not match this configuration will result in an incomplete setup.


## STEP 3:Set Up GitHub Integration

1. **Install Git**:
   - If you haven’t installed Git yet, download and install Git from the official website: [Git Download](https://git-scm.com/).
   
2. **Enable Version Control in Rust Rover**:
   - Open Rust Rover IDE.
   - Go to the menu bar and click `VCS` > `Enable Version Control Integration`.
   - Choose `Git` from the available options.
   
3. **Initialize Git Repository and Link to GitHub**:
   - Open your terminal in the project directory.
   - Run the following commands to initialize a new Git repository and link it to your GitHub repository:
     ```bash
     git init
     git remote add origin <your-github-repository-url>
     ```
     Replace `<your-github-repository-url>` with the actual URL of the GitHub repository you created.

4. **Add, Commit, and Push Code to GitHub**:
   - Run the following commands to add your files, commit them, and push the changes to your GitHub repository:
     ```bash
     git add .
     git commit -m "Initial commit"
     git push -u origin main
     ```
   - After these steps, your project will be linked to GitHub and the initial commit will be pushed to the repository.

