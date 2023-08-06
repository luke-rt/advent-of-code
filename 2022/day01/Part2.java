import java.util.Scanner;
import java.io.File;
import java.io.FileNotFoundException;

public class Part2 {
    public static void main(String[] args) throws FileNotFoundException {
        int first = 0;
        int second = 0;
        int third = 0;
        int n = 0;

        Scanner fin = new Scanner(new File("/Users/luke/dev/advent-of-code/2022/day01/day01.txt"));

        while(fin.hasNextLine()) {
            String ln = fin.nextLine();

            if(ln.equals("")) {
                if(n > first) {
                    third = second;
                    second = first;
                    first = n;
                } else if(n > second) {
                    third = second;
                    second = n;
                } else if(n > third) {
                    third = n;
                }
                n = 0;
            } else {
                n += Integer.parseInt(ln);
            }
        }

        fin.close();

        System.out.println(first + second + third);
    }

}
