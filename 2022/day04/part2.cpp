#include <fstream>
#include <iostream>
#include <sstream>

int main(void) {
	int total = 0;

	std::ifstream fin;
	fin.open("day04.txt");

	std::string line;
	while (fin >> line) {
		std::string first_elf, second_elf;
		std::stringstream comma(line);

		std::getline(comma, first_elf, ',');
		std::getline(comma, second_elf, ',');

		std::string first_elf_1, first_elf_2, second_elf_1, second_elf_2;

		std::stringstream endash1(first_elf);
		std::getline(endash1, first_elf_1, '-');
		std::getline(endash1, first_elf_2, '-');

		std::stringstream endash2(second_elf);
		std::getline(endash2, second_elf_1, '-');
		std::getline(endash2, second_elf_2, '-');

		int a, b, x, y;

		// (a, b) (x, y)

		a = stoi(first_elf_1);
		b = stoi(first_elf_2);
		x = stoi(second_elf_1);
		y = stoi(second_elf_2);

		if(x <= b && y >= a) {
			total++;
		} else if(b <= x && a >= y) {
			total++;
		}
	}

	std::cout << total;
}
