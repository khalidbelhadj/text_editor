#include <stdio.h>

int main() {
    int a = 10;
    int b = 5;
    
    if (a > b) {
        printf("Hello, world!\n");
    } else {
        for (int i = 0; i < 5; i++) {
            printf("Loop iteration: %d\n", i);
        }
    }
    
    int x = 3;
    int y = 7;
    
    while (x != y) {
        x++;
    }
    
    int numbers[] = {1, 2, 3, 4, 5};
    int sum = 0;
    
    for (int i = 0; i < sizeof(numbers) / sizeof(numbers[0]); i++) {
        sum += numbers[i];
    }
    
    printf("Sum of numbers: %d\n", sum);
    
    char name[10];
    printf("Enter your name: ");
    scanf("%s", name);
    printf("Hello, %s!\n", name);
    
    return 0;
}

void printMessage() {
    printf("This is a random function.\n");
}

typedef struct {
    int x;
    int y;
} Point;

Point createPoint(int x, int y) {
    Point p;
    p.x = x;
    p.y = y;
    return p;
}

void displayPoint(Point p) {
    printf("Point: (%d, %d)\n", p.x, p.y);
}

int add(int a, int b) {
    return a + b;
}

int multiply(int a, int b) {
    return a * b;
}

float divide(float a, float b) {
    return a / b;
}

int factorial(int n) {
    if (n <= 1) {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}

int findMax(int arr[], int size) {
    int max = arr[0];
    for (int i = 1; i < size; i++) {
        if (arr[i] > max) {
            max = arr[i];
        }
    }
    return max;
}

void swap(int *a, int *b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

int gcd(int a, int b) {
    if (b == 0) {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

typedef enum {
    RED,
    GREEN,
    BLUE
} Color;

void printColor(Color color) {
    switch (color) {
        case RED:
            printf("Red\n");
            break;
        case GREEN:
            printf("Green\n");
            break;
        case BLUE:
            printf("Blue\n");
            break;
        default:
            printf("Invalid color\n");
    }
}

int getRandomNumber() {
    return rand() % 100;
}

void printRandomNumbers() {
    for (int i = 0; i < 5; i++) {
        printf("%d\n", getRandomNumber());
    }
}
