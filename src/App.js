import { Button } from 'antd';
import { GithubOutlined } from '@ant-design/icons';
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <p>
          <code>coming soon</code>
        </p>
        <Button type="primary" href="https://github.com/khuedoan">
          <GithubOutlined />
          GitHub
        </Button>
      </header>
    </div>
  );
}

export default App;
