#include <stdio.h>
#include <stdlib.h>
#include <string.h>
typedef struct data
{
    char cs[10];
    char name[10];
    int address;
    int length;
} Data;
Data *array = NULL;
int size = 0;
Data *new(char *cs, char *name, int address, int length)
{
    Data *data = (Data *)malloc(sizeof(Data));
    if (data == NULL)
    {
        fprintf(stderr, "Error: Could not allocate memory for data\n");
        return NULL;
    }
    strcpy(data->cs, cs);
    strcpy(data->name, name);
    data->address = address;
    data->length = length;
    return data;
}

Data *add2array(Data *data)
{
    Data *temp = (Data *)realloc(array, (size + 1) * sizeof(Data));
    if (temp == NULL)
    {
        fprintf(stderr, "Error: Could not reallocate memory for data\n");
        return NULL;
    }
    array = temp;
    array[size++] = *data;
    return array;
}
void freeArray()
{
    free(array);
}

FILE *openFile(char *filename, char *mode)
{
    FILE *file = fopen(filename, mode);
    if (file == NULL)
    {
        fprintf(stderr, "Error: Could not open file %s\n", filename);
        return NULL;
    }
    return file;
}
void closeFile(FILE *file)
{
    fclose(file);
}

int main(int argc, char *argv[])
{
    if (argc < 4)
    {
        fprintf(stderr, "Error: Not enough arguments\n");
        return 1;
    }
    int start_address = strtol(argv[1], NULL, 16);
    int start_check = 0;
    int length_check = 0;
    for (int i = 2; i < argc; i++)
    {
        FILE *file = openFile(argv[i], "r");
        if (file == NULL)
        {
            return 1;
        }
        char line[1024];
        while (fgets(line, sizeof(line), file) != NULL)
        {
            if (line[0] == 'H')
            {
                char cs[10];
                int address, length;
                sscanf(line, "H%s %06x %06x", cs, &address, &length);
                if (start_check == 0 && length_check == 0)
                {
                    start_check = start_address;
                    address = start_address;
                    length_check = length;
                }
                else
                {
                    start_check += length_check;
                    address = start_check;
                    length_check = length;
                }
                Data *data = new (cs, "", address, length);
                if (add2array(data) == NULL)
                {
                    fprintf(stderr, "Error: Could not add data to array\n");
                    return -1;
                }
            }
            else if (line[0] == 'D')
            {
                char *ptr = line + 1;
                while (*ptr)
                {
                    char name[10];
                    int address;
                    if (sscanf(ptr, "%[^0-9]%06x", name, &address) != 2)
                        break;
                    address += start_check;
                    Data *data = new ("", name, address, 0);
                    if (add2array(data) == NULL)
                    {
                        fprintf(stderr, "Error: Could not add data to array\n");
                        return -1;
                    }
                    ptr += strlen(name) + 6;
                }
            }
        }
        closeFile(file);
    }
    for (int i = 0; i < size; i++)
    {
        Data data = array[i];
        char length[10];
        if (data.length == 0){
            sprintf(length, "%s", "");
        } else {
            sprintf(length, "%06X", data.length);
        }
        printf("%10s  %6s %06X  %6s\n", data.cs, data.name, data.address, length);
    }
    freeArray();
    return 0;
}