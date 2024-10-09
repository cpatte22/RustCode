#include <stdio.h>
/*
int main() {
   printf("This is the C program");
   return 0;
}
*/
void cFoo(){

   //Example of I/O in C ---NOT WORKING---
   int num1, num2;
   scanf("Enter num1 (C): %d", num1);
   scanf("Enter num2 (C): %d", num2);

   int sum = num1 + num2;
   printf("The sume of the numbers is %d", sum);
   printf("This is the C Program");
}