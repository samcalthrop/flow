import classes from './Home.module.css';
import { Screen } from '../Screen';
import { useState } from 'react';

export const Home = (): JSX.Element => {
    return (
      <Screen>
        <h1>Welcome Home!</h1>
      </Screen>
    );
}
