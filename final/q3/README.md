# 3 After Election Day [20 marks]

Credit: https://github.com/phayes/tallystick (MIT OR Apache 2.0-licensed)

An important part of running an election is counting the collected votes.
In the q3 directory you will find some code from tallystick which tallies votes.
I’ve added a q3/src/main.rs file which is a simple test driver.
It manufactures 10 million weighted votes and adds them to a tally.
Your task is to parallelize this code to use 4 threads.
As in Assignment 4, you are not to trivialize the problem.
In this case, that means that you have to have 10 million calls to the add function and you have to produce the same answer; you can’t hard-code the final results in the vector.
You may, however, create more tally objects and split the iteration into different threads, and you may start the iterations partway through the 10 million votes, hardcoding the start point (though there are other valid approaches as well).
On ecetesla0, my solution achieves a speedup of 2.87× over the original code (and 3.1× over the code with 1 thread and locks). For full marks, achieve a speedup of 2.5× over the original code.
