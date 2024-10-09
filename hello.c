#include <stdio.h>
#include <string.h>
/*
int main() {
   printf("This is the C program");
   return 0;
}
*/

//Function to encrypt a string
void encrypt(char* str, int shift) 
{
    int i = 0;
    while (str[i] != '\0') {
        //Shift each character by the shift value
        str[i] = str[i] + shift;
        i++;
    }
}

void cFoo(){

   printf("This is the C Program");
   //Example of I/O in C ---NOT WORKING---
   int num1, num2;
   scanf("Enter num1 (C): %d", num1);
   scanf("Enter num2 (C): %d", num2);

   int sum = num1 + num2;
   printf("The sume of the numbers is %d", sum);
   //-------------------------------------

   char str[100];
   int shift;

   printf("Enter a string to encrypt: ");
   fgets(str, sizeof(str), stdin);

    //remove newline character from fgets input
   str[strcspn(str, "\n")] = '\0';

    //get shift value
   printf("Enter shift value (e.g., 1): ");
   scanf("%d", &shift);

   encrypt(str, shift);
   printf("Encrypted string: %s\n", str);   
}

