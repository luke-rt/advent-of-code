#include <fstream>
#include <iostream>
using namespace std;


int main(void) {
	int score = 0;
	char n, m;

	ifstream fin;
	fin.open("day02.txt");

	while(fin >> n >> m) {
		if(m == 'X') { // lose
			if(n == 'A') score += 3;
			else if(n == 'B') score += 1;
			else score += 2;
		}
		else if(m == 'Y') { // draw
			score += 3;

			if(n == 'A') score += 1;
			else if(n == 'B') score += 2;
			else score += 3;
		}
		else { // win
			score += 6;

			if(n == 'A') score += 2;
			else if(n == 'B') score += 3;
			else score += 1;
		}
	}

	fin.close();

	cout << score << endl;
}
