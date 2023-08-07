#include <iostream>
#include <string>
#include <fstream>
using namespace std;

int main(void) {
	int total = 0;
	string rucksack;

	ifstream fin;
	fin.open("day03.txt");

	while(fin >> rucksack) {
		string comp_one = rucksack.substr(0, rucksack.length()/2);
		string comp_two = rucksack.substr(rucksack.length()/2);

		for(char ch : comp_one) {
			if(comp_two.find(ch) != string::npos) {
				if(islower(ch)) total += int(ch) - 96;
				else total += int(ch) - 38;
				break;
			}
		}
	}
	fin.close();

	cout << total << endl;
}
