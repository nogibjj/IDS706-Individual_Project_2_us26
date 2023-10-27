## Rust CLI Binary with SQLite

[![Rust CI/CD](https://github.com/nogibjj/IDS706-Individual_Project_2_us26/actions/workflows/cicd.yml/badge.svg)](https://github.com/nogibjj/IDS706-Individual_Project_2_us26/actions/workflows/cicd.yml)

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS-Week5_MiniProject_us26/blob/main/images/schema.png" alt="schema">
</p>


### Overview

### Code Description

1. *** Rust source code ***
Rust Initiation using cargo init

initialize a new Rust project by running ‘cargo init` inside the directory, it will set up a new Rust project by:

Creating a Cargo.toml file, which contains configuration data, dependencies, and other metadata about the Rust project.

Creating a src directory with a main.rs file for binary projects or lib.rs for libraries.

Here's how the toml file looks:

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/packages.png" alt="schema">
</p>

2. *** Usage of Github Copilot ***
    2.1 Creating main.rs
    The rust file main.rs has been created with the help of copilot. The week5 mini project has the python code to perform CRUD operation using sqlite. The copilot helped me in translating it into rust 
2. create.py
    This script is used for load and transform. A databased called 'ranking.db' with a table named 'data' is created and a csv file is loaded into that table.

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/create.png" alt="schema">
</p>

3. To begin running the code after successfully building using 
      - Cargo build 
  Run the command 
      - cargo run main.rs
The ouput from this command will be :

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/run.png" alt="schema">
</p>

Here we can directly input the sql command and interact with the database

4. read.py
  The queries used are :
    - SELECT "Name of University" FROM universities WHERE
      "Location" == "United States"
    - SELECT "Name of University", "No of student per staff" FROM
      universities WHERE "No of student per staff" > 40.0
    - SELECT "Name of University", "No of student per staff" FROM
        universities WHERE ("No of student per staff" < 40.0) AND ("Location" == "Canada")

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/read1.png" alt="schema">
</p>

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/read2.png" alt="schema">
</p>

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/read3.png" alt="schema">
</p>
    
5. update.py
    Updating of tuple values already present in the table.

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/update.png" alt="schema">
</p>

6. delete.py
    Deletion of data present in the table. The query used is :
      DELETE FROM universities WHERE "Industry Income Score" < 90.0

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/delete.png" alt="schema">
</p>



6. test_graph.py
    ** pd.read_sql_query **  is used for creating visualisation.
    It is a function used to read SQL query or database table into DataFrame.
   
7. Makefile with the following actions:

	- build:
<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/build.png" alt="schema">
</p>

	- test:
<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/test.png" alt="schema">
</p>

	- format: 
<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/format.png" alt="schema">
</p>

  - lint: 
<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/lint.png" alt="schema">
</p>


8.Created GitHub Actions that performs all four Makefile commands with badges for each one in the README.md

##### Action include the general CI/CD process in test.yml file, which automatically generate the graph and markdown

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS-Week5_MiniProject_us26/blob/main/images/ci_cd.png" alt="cicd">
</p>

## Visualization 
#### Visualization Created using sql database using pandas.read_sql_query (https://pandas.pydata.org/docs/reference/api/pandas.read_sql_query.html)

##### Count of top universities vs mean industry income score 

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_1_us26/blob/main/output_graph/visualization.png" alt="visualization">
</p>	
