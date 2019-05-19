import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.util.Scanner;
import java.util.Arrays;

public class MainJ {
    public static void main(String[] args) {
        int N = 100000000;
        int[] xs = new int[N];
        int random = N;
        for (int i = 0; i < N; i++) {
            random ^= random << 13;
            random ^= random >> 17;
            random ^= random << 5;
            xs[i] = random;
        }
        for (int i = 0; i < 100; i++) {
            long start = System.currentTimeMillis();
            double res = average(xs);
            long end = System.currentTimeMillis();
            if (i % 10 == 0) {
                System.out.println(res + " " + (end - start));
            }
        }
    }

    private static double average(int[] data) {
        int sum = 0;
        for (int i = 0; i < data.length; i++) {
            sum += data[i];
        }
        return sum * 1.0d / data.length;
    }
}
