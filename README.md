## Rust CLI Binary with SQLite

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS-Week5_MiniProject_us26/blob/main/images/schema.png" alt="schema">
</p>


### Overview

This repo has been created by forked from (https://github.com/nogibjj/sqlite-lab). I have used world university ranking csv file and loaded it into 'ranking.db' database under the table name 'universities'. Using **CRUD** operation to interacts with a SQL database.

Here is an overview of CRUD operations:
		
  CRUD stands for Create, Read, Update, and Delete, which are the four basic operations for managing data in a database or data storage system. These operations are essential for interacting with and manipulating data within an application or database. Here's a brief overview of each CRUD operation:

1. **Create (C)**:
   - Create is the operation used to add new data or records to a database.
   - It typically involves inserting a new row or document into a database table or collection.
   - In SQL, you use the `INSERT INTO` statement to create new records in a table.
   - In NoSQL databases, you often use methods like `insertOne` or `insertMany` to add documents to a collection.

2. **Read (R)**:
   - Read is the operation used to retrieve data from a database.
   - It involves querying the database to fetch existing records based on specific criteria.
   - In SQL, you use the `SELECT` statement to read data from a table.
   - In NoSQL databases, you use various query methods to retrieve documents that match your criteria.

3. **Update (U)**:
   - Update is the operation used to modify existing data in a database.
   - It typically involves changing the values of one or more fields in an existing record.
   - In SQL, you use the `UPDATE` statement to update data in a table.
   - In NoSQL databases, you use methods like `updateOne` or `updateMany` to modify documents in a collection.

4. **Delete (D)**:
   - Delete is the operation used to remove data from a database.
   - It can involve deleting specific records or entire rows from a table.
   - In SQL, you use the `DELETE` statement to remove data from a table.
   - In NoSQL databases, you use methods like `deleteOne` or `deleteMany` to delete documents from a collection.


### Code Description

1. create.py
    This script is used for load and transform. A databased called 'ranking.db' with a table named 'universities' is created and a csv file is loaded into that table.

2. To begin running the code after successfully building using 
      - Cargo build 
  Run the command 
      - cargo run main.rs
The ouput from this command will be :

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/run.png" alt="schema">
</p>

Here we can directly input the sql command and interact with the database

2. read.py
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
    
3. update.py
    Updating of tuple values already present in the table.

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/update.png" alt="schema">
</p>


4. delete.py
    Deletion of data present in the table. The query used is :
      DELETE FROM universities WHERE "Industry Income Score" < 90.0

<p align="center">
  <img width="600" src="https://github.com/nogibjj/IDS706-Individual_Project_2_us26/blob/main/images/delete.png" alt="schema">
</p>

6. test_graph.py
    ** pd.read_sql_query **  is used for creating visualisation.
    It is a function used to read SQL query or database table into DataFrame.
   
6. Makefile with the following:

	- install: using requirements.txt file to install required packages

	- test:

	- format: using black formatter

  - lint: using ruff 


7.Created GitHub Actions that performs all four Makefile commands with badges for each one in the README.md

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
