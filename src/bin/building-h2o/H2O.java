import java.util.concurrent.Executors;
import java.util.concurrent.ExecutorService;

public class H2O {

    public static void main(String[] args) {
        final Runnable releaseHydrogen = () -> System.out.print("H");
        final Runnable releaseOxygen = () -> System.out.print("O");

        ExecutorService executorService = Executors.newFixedThreadPool(100);
        final H2O h2o = new H2O();
        executorService.execute(() -> {
            try {
                h2o.oxygen(releaseOxygen);
            } catch (InterruptedException e) {
            }
        });
        executorService.execute(() -> {
            try {
                h2o.oxygen(releaseOxygen);
            } catch (InterruptedException e) {
            }
        });
        executorService.execute(() -> {
            try {
                h2o.hydrogen(releaseHydrogen);
            } catch (InterruptedException e) {
            }
        });
        executorService.execute(() -> {
            try {
                h2o.hydrogen(releaseHydrogen);
            } catch (InterruptedException e) {
            }
        });
        executorService.execute(() -> {
            try {
                h2o.hydrogen(releaseHydrogen);
            } catch (InterruptedException e) {
            }
        });
        executorService.execute(() -> {
            try {
                h2o.hydrogen(releaseHydrogen);
            } catch (InterruptedException e) {
            }
        });

        executorService.execute(() -> {
            try {
                h2o.hydrogen(releaseHydrogen);
            } catch (InterruptedException e) {
            }
        });
        executorService.execute(() -> {
            try {
                h2o.hydrogen(releaseHydrogen);
            } catch (InterruptedException e) {
            }
        });
        executorService.execute(() -> {
            try {
                h2o.oxygen(releaseOxygen);
            } catch (InterruptedException e) {
            }
        });
        executorService.execute(() -> {
            try {
                h2o.hydrogen(releaseHydrogen);
            } catch (InterruptedException e) {
            }
        });
        executorService.execute(() -> {
            try {
                h2o.hydrogen(releaseHydrogen);
            } catch (InterruptedException e) {
            }
        });
        executorService.execute(() -> {
            try {
                h2o.oxygen(releaseOxygen);
            } catch (InterruptedException e) {
            }
        });
        executorService.shutdown();
    }

    public H2O() {

    }

    private final java.util.concurrent.Semaphore h = new java.util.concurrent.Semaphore(2);
    private final java.util.concurrent.Semaphore o = new java.util.concurrent.Semaphore(0);

    public void hydrogen(Runnable releaseHydrogen) throws InterruptedException {
        h.acquire();
        // releaseHydrogen.run() outputs "H". Do not change or remove this line.
        releaseHydrogen.run();
        o.release();
    }

    public void oxygen(Runnable releaseOxygen) throws InterruptedException {
        o.acquire(2);
        // releaseOxygen.run() outputs "O". Do not change or remove this line.
        releaseOxygen.run();
        h.release(2);
    }
}
