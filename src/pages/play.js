import React, { useState, useEffect } from "react";
import * as execute from '../contract/execute';
import { useConnectedWallet } from "@terra-money/wallet-provider";
import LoadingIndicator from '../components/LoadingIndicator';

const Play = () => {
    const connectedWallet = useConnectedWallet();
    // Configure this time as you want
    const playTime = 15;

    const [time, setTime] = useState(playTime);
    const [gameOver, setGameOver] = useState(false);
    // We use this to track where the target is on the screen
    const [targetPosition, setTargetPosition] = useState({ top: "15%", left: "50%" });
    const [loading, setLoading] = useState(false);
    const [score, setScore] = useState(0);

    // Every second we're going to lower the value of time
    useEffect(() => {
        const unsubscribe = setInterval(() => {
            setTime(time => time > 0 ? time - 1 : 0);
        }, 1000);
        return unsubscribe;
    }, []);

    useEffect(() => {
        if (time === 0) {
            setTargetPosition({ display: 'none' });
            // Show alert to let user know it's game over
            alert(`Game Over! Your score is ${score}. Please confirm transaction to submit score.`);
            submitScore();
        }
    }, [time]);

    const submitScore = async () => {
        if (connectedWallet && connectedWallet.network.name === 'testnet') {
            setLoading(true);
            // This will return the transaction object on confirmation 
            const tx = await execute.setScore(connectedWallet, score);
            console.log(tx);
            // Once the transaction is confirmed, we let the user know
            alert('Score submitted!');
            setLoading(false);
            window.location.href = '/leaderboard';
        }
    };

    const handleClick = () => {
        let audio = new Audio("/cat_sound.mp3");

        // Don't let it get too loud
        audio.volume = 0.2;
        audio.play();

        setScore(score => score + 1);

        // Play around with this to control bounds
        setTargetPosition({
            top: `${Math.floor(Math.random() * 80)}%`,
            left: `${Math.floor(Math.random() * 80)}%`
        });
    };

    return (
        <div className="game-board-container">
            <div className="play-container">
                <span className="play-header">Score: {score} </span>
                <span className="scratch-header">SCRATCH!</span>
                <span className="play-header">Time left: {time}s</span>
            </div>

            {/* Render loading or game container */}
            {loading ? (
                <LoadingIndicator />
            ) : (
                <div className="game-container">
                    {/* Set the image. It's loaded from the public folder. */}
                    <img src={"target_cat4.png"} id="target" alt="Target" style={{ ...targetPosition }} onClick={handleClick} />
                    <img src="ninja_cat.png" id="ninjacat-img" alt="NinjaCat" />
                </div>
            )}
        </div>
    );
};

export default Play;