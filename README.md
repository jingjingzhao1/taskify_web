# taskify_web
Name: Jingjing Zhao
Project name: Taskify

Taskify is a web application built with Actix, allowing users to manage their tasks efficiently. It provides features to create, update, and delete tasks. The application uses a RESTful API to handle requests and interacts with a database(rusqlite) to store task data.

Functionality

The main functionality of Taskify includes:

Viewing a list of all tasks.
Creating new tasks with a title, description.
Updating task information such as title, description, and progress.
Deleting tasks from the database.

Build and Run

To build and run the Taskify project, follow these steps:

Clone the repository: git clone https://github.com/jingjingzhao1/taskify_web.git
Navigate to the project directory: cd taskify_web/taskify
Install dependencies: cargo build
Start the application: cargo run
By default, the application will be accessible at http://localhost:8080.

API Testing with Postman

Example

Here's an example illustrating the operation of the Taskify application:

User visits the Taskify homepage at http://localhost:8080.
User clicks on the "Add" button to create a new task.
User fills in the task details (title, description) and submits the form.
The new task is created and added to the list of tasks.
User can update the task details using the update button(home page).
User can delete the task using delete button(home page).

I am satisfied with the overall functionality of the Taskify project. 

License

Taskify is released under the MIT License.