import { Breadcrumb, Button, Layout, Menu, Space } from 'antd';
import { GithubFilled, LinkedinFilled } from '@ant-design/icons';
import './App.css';

const { Header, Footer, Content } = Layout;

function App() {
  return (
    <Layout>
      <Header>
        <div className="logo" />
        <Menu theme='dark' mode="horizontal" defaultSelectedKeys={['1']}>
          <Menu.Item key="1">Khue Doan</Menu.Item>
        </Menu>
      </Header>
      <Content style={{ padding: '0 50px' }}>
        <Breadcrumb style={{ margin: '16px 0' }}>
          <Breadcrumb.Item>Home</Breadcrumb.Item>
        </Breadcrumb>
        <div className="site-layout-content">
          Coming soon
        </div>
      </Content>
      <Footer style={{ textAlign: 'center' }}>
        <Space>
          <Button icon={<GithubFilled />} href="https://github.com/khuedoan" />
          <Button icon={<LinkedinFilled />} href="https://linkedin.com/in/khuedoan" />
        </Space>
      </Footer>
    </Layout>
  );
}

export default App;
