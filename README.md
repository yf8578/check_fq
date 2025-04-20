# Check fastq--Rust  

When I was a senior student in college, my first task during my company internship was to write a script to check fastq format.  
At that time, I used Python and wrote a simple script. However, I later found that Python was too slow, especially when processing large files. The task was eventually left unresolved. Now I want to re-implement this functionality using Rust.  
---
Expected features:    
- Checking  
1. Check if the fastq file format is correct
    - Four lines per group    
    - The first line starts with @, the second line is the sequence, the third line starts with +, and the fourth line is the quality value  
    - The sequence and quality value have the same length  
- Error handling  
1. Implement error handling by outputting problematic sequences and their corresponding information to a new file  
