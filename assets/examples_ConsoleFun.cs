using System.Diagnostics;
using System.Drawing;

namespace ConsoleFun;
class Program
{
    static async Task Main(string[] args)
    {
        var game = new Game(rows: 10, cols: 100);
        await game.Start();
    }
}

public class Ball
{
    public Point Position { get; set; }
    private bool BallGoingDown = true;
    private bool BallGoingRight = true;
    private int ScreenX;
    private int ScreenY;
    public readonly char BallCharacter = '+';

    public Ball(Point ballPosition, int screenX, int screenY)
    {
        Position = ballPosition;
        ScreenX = screenX;
        ScreenY = screenY;
    }

    public void Move()
    {
        var ballx = 0;
        var bally = 0;
        if (BallGoingDown)
        {
            bally++;
        }
        else
        {
            bally--;
        }
        if (BallGoingRight)
        {
            ballx++;
        }
        else
        {
            ballx--;
        }

        Position = new(Position.X + ballx, Position.Y + bally);

        if (Position.Y == 0 || Position.Y == ScreenY - 1)
        {
            BallGoingDown = !BallGoingDown;
        }

        if (Position.X == 0 || Position.X == ScreenX - 1)
        {
            BallGoingRight = !BallGoingRight;
        }
    }
}

public class Game
{
    char[,] Screen;
    Ball Ball;
    Random Random = new();

    public Game(int rows, int cols)
    {
        Screen = new char[cols, rows];
        Ball = new(new() { X = Random.Next(0, cols), Y = Random.Next(0, rows) }, cols, rows);
        FillScreen('_');
    }

    public async Task Start()
    {
        while (true)
        {
            Turn();
            await Task.Delay(33);
        }
    }

    private void FillScreen(char c)
    {
        for (int i = 0; i < Screen.GetLength(0); i++)
        {
            for (int j = 0; j < Screen.GetLength(1); j++)
            {
                Screen[i, j] = c;
            }
        }
    }

    private void Turn()
    {
        Ball.Move();
        for (int i = 0; i < Screen.GetLength(0); i++)
        {
            for (int j = 0; j < Screen.GetLength(1); j++)
            {
                Console.SetCursorPosition(i, j);

                if (Ball.Position.X == i && Ball.Position.Y == j)
                {
                    Console.Write(Ball.BallCharacter);
                }
                else
                {
                    Console.Write(Screen[i, j]);
                }
            }
        }
    }
}

