import java.util.function.IntConsumer;

public class FizzBuzz {

    public static void main(String[] args) {
    }

    private int n;

    public FizzBuzz(int n) {
        this.n = n;
    }

    private volatile int i = 1;
    private final java.util.concurrent.Semaphore f = new java.util.concurrent.Semaphore(0);
    private final java.util.concurrent.Semaphore b = new java.util.concurrent.Semaphore(0);
    private final java.util.concurrent.Semaphore fb = new java.util.concurrent.Semaphore(0);
    private final java.util.concurrent.Semaphore num = new java.util.concurrent.Semaphore(1);

    // printFizz.run() outputs "fizz".
    public void fizz(Runnable printFizz) throws InterruptedException {
        while (true) {
            f.acquire();
            if (i > n) {
                break;
            }
            printFizz.run();
            ++i;
            release();
        }
    }

    // printBuzz.run() outputs "buzz".
    public void buzz(Runnable printBuzz) throws InterruptedException {
        while (true) {
            b.acquire();
            if (i > n) {
                break;
            }
            printBuzz.run();
            ++i;
            release();
        }
    }

    // printFizzBuzz.run() outputs "fizzbuzz".
    public void fizzbuzz(Runnable printFizzBuzz) throws InterruptedException {
        while (true) {
            fb.acquire();
            if (i > n) {
                break;
            }
            printFizzBuzz.run();
            ++i;
            release();
        }
    }

    // printNumber.accept(x) outputs "x", where x is an integer.
    public void number(IntConsumer printNumber) throws InterruptedException {
        while (true) {
            num.acquire();
            if (i > n) {
                break;
            }
            printNumber.accept(i);
            ++i;
            release();
        }
    }

    public void release() {
        if (i > n) {
            fb.release();
            f.release();
            b.release();
            num.release();
        }
        int m3 = i % 3;
        int m5 = i % 5;
        if (m3 == 0 && m5 == 0) {
            fb.release();
        } else if (m3 == 0 && m5 != 0) {
            f.release();
        } else if (m3 != 0 && m5 == 0) {
            b.release();
        } else {
            num.release();
        }
    }
}
