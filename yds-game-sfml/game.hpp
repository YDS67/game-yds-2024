#ifndef GAME_HPP // include guard
#define GAME_HPP

#include <sstream>
#include <iomanip>
#include <iostream>
#include <cmath>

#include <SFML/Graphics.hpp>
#include <SFML/Window.hpp>
#include <SFML/System.hpp>

#include "settings.hpp"

// game context

class Game
{
public:
    sf::RenderWindow window;
    sf::Clock clock1;
    sf::Clock clock2;
    float current_time;
    float frame_time;
    int frame_count;
    int seconds_count;
    int minutes_count;
    int current_fps;
    sf::Font font;

    Game()
    {
        // create the window
        window.create(sf::VideoMode(WINDOW_WIDTH, WINDOW_HEIGHT),
                      WINDOW_TITLE, sf::Style::Default);
        window.setFramerateLimit(TARGET_FPS);
        window.setKeyRepeatEnabled(false);
        window.setActive();

        new_game();
    };

    void new_game()
    {
        font.loadFromFile("resources/brohoney.ttf");

        clock1.restart();
        clock2.restart();

        current_fps = TARGET_FPS;

        frame_count = 0;
        seconds_count = 0;
        minutes_count = 0;

    };

    void game_update()
    {
        // find FPS and time
        update_time();
    };

    void game_draw()
    {
        // fill screen with color
        window.clear(sf::Color(135, 206, 235, 255));

        // show FPS and time
        show_time();

        // end the current frame
        window.display();
    };

    void game_check_events()
    {
        // check all the window's events that were triggered since the last iteration of the loop
        sf::Event event;
        while (window.pollEvent(event))
        {
            // "close requested" event: we close the window
            if (event.type == sf::Event::Closed)
                window.close();

            if (event.type == sf::Event::KeyPressed)
            {
                // close at the press of Esc button
                if (event.key.code == sf::Keyboard::Escape)
                    window.close();
            };
        };
    };

    void game_run()
    {
        // run the program as long as the window is open
        while (window.isOpen())
        {
            game_update();
            game_check_events();
            game_draw();
        }
    };

    void update_time()
    {
        current_time = clock1.getElapsedTime().asSeconds();
        frame_time = clock2.restart().asSeconds();

        frame_count += 1;

        if (current_time >= 1.0)
        {
            seconds_count += 1;
            current_fps = frame_count;
            frame_count = 0;
            clock1.restart();
        };

        if (seconds_count >= 60)
        {
            minutes_count += 1;
            seconds_count = 0;
        };
    };

    void show_time()
    {
        sf::RectangleShape text_bg;
        text_bg.setSize(sf::Vector2f(200, 150));
        text_bg.setPosition(sf::Vector2f(20, 20));
        text_bg.setFillColor(sf::Color(150, 150, 150, 255));
        text_bg.setOutlineThickness(3.f);
        text_bg.setOutlineColor(sf::Color(50, 50, 50, 255));
        window.draw(text_bg);

        std::ostringstream display_fps;
        display_fps << "FPS: " << current_fps;
        sf::Text text(display_fps.str(), font, FONT_SIZE);
        text.setFillColor(sf::Color::Black);
        text.setPosition(30.f, 30.f);
        text.setOrigin(0.f, 0.f);
        window.draw(text);

        std::ostringstream display_time;
        display_time << "Time: " << minutes_count << ":" << seconds_count; // << ":" << ms_count;
        text = sf::Text(display_time.str(), font, FONT_SIZE);
        text.setFillColor(sf::Color::Black);
        text.setPosition(30.f, 55.f);
        text.setOrigin(0.f, 0.f);
        window.draw(text);

    };
};

#endif /* GAME_HPP */