public class Main {
    public static void main(String[] args) {
        MyLinkedList list = new MyLinkedList();
        list.addAtTail(0);
        list.addAtTail(1);
        list.addAtTail(2);
        list.addAtTail(3);
        System.out.println(list.get(3));
        list.addAtHead(1);
        list.addAtHead(2);
        list.addAtHead(3);
        System.out.println(list.get(3));

        printList(list);
        list.deleteAtIndex(6);
        printList(list);
        list.addAtTail(4);
        printList(list);

        list.deleteAtIndex(3);
        printList(list);

        list.deleteAtIndex(0);
        printList(list);
        list.addAtHead(4);
        printList(list);

        list.addAtIndex(1, 3);
        printList(list);
    }

    public static void printList(MyLinkedList list) {
        for (int i = 0; i < list.size(); i++) {
            System.out.print(list.get(i) + " ");
        }
        System.out.println();
    }
}

class MyLinkedList {
    private int length;
    private Node head;
    private Node tail;

    public MyLinkedList() {
        this.head = new Node(-1);
        this.tail = head;
    }

    public int get(int index) {
        if (index >= length) {
            return -1;
        }
        Node curr = this.head.next;
        int count = 0;
        while (count < index) {
            curr = curr.next;
            count++;
        }
        return curr.val;
    }

    public void addAtHead(int val) {
        Node node = new Node(val);
        node.next = this.head.next;
        this.head.next = node;
        if (head == tail) {
            this.tail = this.head.next;
        }
        this.length++;
    }

    public void addAtTail(int val) {
        this.tail.next = new Node(val);
        this.tail = this.tail.next;
        this.length++;
    }

    public void addAtIndex(int index, int val) {
        if (index > length) {
            return;
        }
        if (index == length) {
            addAtTail(val);
            return;
        }
        if (index == 0) {
            addAtHead(val);
            return;
        }
        int count = 0;
        Node curr = head;
        while (count < index) {
            curr = curr.next;
            count++;
        }
        Node node = new Node(val);
        node.next = curr.next;
        curr.next = node;
        length++;
    }

    public void deleteAtIndex(int index) {
        if (index >= length) {
            return;
        }
        Node prev = this.head;
        Node curr = this.head.next;
        int count = 0;
        while (count < index) {
            prev = curr;
            curr = curr.next;
            count++;
        }
        prev.next = curr.next;
        if (this.tail == curr) {
            this.tail = prev;
        }
        this.length--;
    }

    public int size() {
        return this.length;
    }
}

class Node {
    int val;
    Node next;

    Node(int val) {
        this.val = val;
    }
}
