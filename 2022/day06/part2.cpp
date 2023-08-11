#include <fstream>
#include <iostream>
#include <vector>

int main(void) {
	std::ifstream fin;
	fin.open("day06.txt");

	std::string line;
	fin >> line;
	fin.close();

	int i = 0;
	std::vector<char> tmp;

	while(tmp.size() < 14) {
		char ch = line[i];
		std::vector<char>::iterator iter = std::find(tmp.begin(), tmp.end(), ch);

		if(iter != tmp.end()) {
			for(int j = 0; j <= iter - tmp.begin(); ++j) {
				tmp.erase(tmp.begin());
			}
		}

		tmp.push_back(ch);

		++i;
	}

	std::cout << i << std::endl;

}

