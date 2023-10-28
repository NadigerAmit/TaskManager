Git command to be used:
1. Install SQLite3 Development Library:
   sudo apt --fix-broken install
   sudo apt-get install libsqlite3-dev
2. Clone the git repository
   git clone https://github.com/NadigerAmit/TaskManager.git
3. Run the application
   cargo run 

# Todo app 

Below is attempt to build the Todo application using Rust 
Initially I built it solely using Rusqlite for all CRUD operations, and while it was functional, I was not completely satisfied with the design. The main concern was that all operations were being performed directly on the database and on the main thread, potentially causing performance issues as the number of tasks increased.

BTW the reason for choosing SQLite as the database for the ToDo application is to create a self-sufficient and easy-to-use application. SQLite is a lightweight, serverless, self-contained database engine that operates directly on the application's local storage. There are several advantages to using SQLite in this context such as easy to use , no configuration , self contained , small size , portable in many platforms .

To address the above issues  , I decided to refactor the application to achieve better efficiency and a more modular architecture. Here are the key improvements I made:
The architecture drives like Scalability, Swift User Interaction, Flexibility are kept in mind while designing this ToDo App.

## Local Cache Using HashMap:(Taskmanager.rs)
To improve the response time for CRUD operations, I implemented a local cache using a HashMap. This HashMap holds the tasks in memory, allowing for quick access and manipulation without having to perform repeated database queries. As a result, the user experiences swift interactions with the application. With this the application can handle up to 100,000 tasks efficiently due to the use of in-memory HashMap for quick access. 
Swift User Interaction: By leveraging local cache and asynchronous database operations, the application maintains quick response times, enhancing user experience.

## Separate DB Thread:(dbThread.rs)
To offload heavy database operations from the main thread, I introduced a separate thread dedicated to database operations. When a user initiates a CRUD operation, the task is first processed in the local HashMap, ensuring rapid response times. Simultaneously, the operation is posted asynchronously to the database thread for processing, enhancing overall performance.

## MVVM Architecture:(taskViewModel.rs)
I introduced the MVVM (Model-View-ViewModel) architecture to the application. This approach allowed me to clearly separate concerns and maintain a more organized codebase. The View Model acts as an interface between the main.rs and taskmanager.rs, streamlining communication and enhancing maintainability. The MVVM architecture promotes modularity and allows for easier maintenance and updates in the future.

## UUID for Unique IDs:
To maintain data integrity, I utilized UUIDs (Universally Unique Identifiers) to generate unique IDs for each task. This ensures that tasks are uniquely identifiable across the application and minimizes the risk of ID collisions.
main.rs : Acts as UI 

I think these improvements will significantly enhance the efficiency and usability of the Task Manager application. By leveraging a local cache, as well as asynchronous database operations,I think we can achieved swift user interaction and better scalability to handle a larger number of tasks.

However, I am still seeking ways to enhance the application further. I am currently exploring additional improvements, such as introducing dependency injection for loose coupling, maintaining a separate list of tasks for optimized operations, and creating a database interface to enable seamless switching of database implementations with dbs other than squlite,etc and encrypting the content of db.
