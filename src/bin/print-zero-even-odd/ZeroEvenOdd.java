import java.util.concurrent.Executors;
import java.util.concurrent.ExecutorService;

public class ZeroEvenOdd {

    public static void main(String[] args) {
        final ZeroEvenOdd z = new ZeroEvenOdd(9);
        final IntConsumer intConsumer = new IntConsumer();
        ExecutorService executorService = Executors.newFixedThreadPool(3);
        executorService.execute(() -> {
                try {
                    z.zero(intConsumer);
                } catch (InterruptedException e) {
                }
        });
        executorService.execute(() -> {
                try {
                    z.even(intConsumer);
                } catch (InterruptedException e) {
                }
        });
        executorService.execute(() -> {
             try {
                 z.odd(intConsumer);
             } catch (InterruptedException e) {
             }
        });
        executorService.shutdown();
    }

    static class IntConsumer {
        public void accept(int i) {
            System.out.print(i);
        }
    }


    private int n;

    public ZeroEvenOdd(int n) {
        this.n = n;
    }

    private volatile boolean flag = false;
    private volatile int i = 1;

    // printNumber.accept(x) outputs "x", where x is an integer.
    public void zero(IntConsumer printNumber) throws InterruptedException {
        while (i <= n) {
            synchronized (this) {
                if (flag) {
                    wait();
                } else {
                    printNumber.accept(0);
                    flag = true;
                    notifyAll();
                }
            }
        }
    }

    public void even(IntConsumer printNumber) throws InterruptedException {
        while (i <= n) {
            synchronized (this) {
                if (!flag || i % 2 != 0) {
                    wait();
                } else {
                    printNumber.accept(i);
                    i++;
                    flag = false;
                    notifyAll();
                }
            }
        }
    }

    public void odd(IntConsumer printNumber) throws InterruptedException {
        while (i <= n) {
            synchronized (this) {
                if (!flag || i % 2 == 0) {
                    wait();
                } else {
                    printNumber.accept(i);
                    i++;
                    flag = false;
                    notifyAll();
                }
            }
        }
    }
}
