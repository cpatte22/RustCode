#include <stdio.h>
#include <string.h>

//Function to encrypt a string
void encrypt(char* str, int shift) {
    int i = 0;
    while (str[i] != '\0') {
        //shift each character by the shift value
        str[i] = str[i] + shift;
        i++;
    }
}

void cFoo() {
    printf("This is the C program\n");

    //Example of I/O in C ---WORKING---
    int num1, num2;
    printf("Enter num1 (C): ");
    scanf("%d", &num1);
    printf("Enter num2 (C): ");
    scanf("%d", &num2);
    int sum = num1 + num2;
    printf("The sum of the numbers is %d\n", sum);
    //-------------------------------------

    char str[100];
    int shift;

    //get string input from user
    printf("Enter a string to encrypt: ");
    getchar(); // To consume the leftover newline character from previous input
    fgets(str, sizeof(str), stdin);

    //remove newline character from fgets input
    str[strcspn(str, "\n")] = '\0';

    //get shift value
    printf("Enter shift value: ");
    scanf("%d", &shift);

    encrypt(str, shift);
    printf("Encrypted string: %s\n", str);
}
