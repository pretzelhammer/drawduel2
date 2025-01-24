import 'src/apps/index/index-app.css';
import classes from 'src/apps/index/IndexApp.module.css';
import React from 'react';

export const IndexApp: React.FC = () => {
    return (
        <>
            <h1>draw duel 🎨⚔️</h1>
            <h2>home page</h2>
            <div className={classes['button-group']}>
                <a
                    className={`button ${classes['button__new-game']}`}
                    href="/mini-game"
                >
                    play public mini game
                </a>
            </div>
        </>
    );
};
