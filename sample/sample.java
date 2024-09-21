public class Main{
    public static void main(String[] args){
        printMyName("Jesse");
        printMyAge(25);
    }

    public static void printMyName(String name){
        System.out.println("Sayyyy myyyy naaaameee");
        System.out.println("My name is: " + name);
    }

    public static int printMyAge(int age){
        System.out.println("I am " + age + " years old");
        return age;
    }
}