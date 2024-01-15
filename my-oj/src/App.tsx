import Table from 'react-bootstrap/Table';
import Button from 'react-bootstrap/Button';
import Stack from 'react-bootstrap/Stack';
import Form from 'react-bootstrap/Form';
import React, { useEffect, useState } from 'react';
import ListGroup from 'react-bootstrap/ListGroup';
import { BrowserRouter as Router, Link, Route, Routes, NavLink } from 'react-router-dom';
import Tab from 'react-bootstrap/Tab';
import Tabs from 'react-bootstrap/Tabs';
import 'bootstrap/dist/css/bootstrap.min.css';
import axios from 'axios';
import Nav from 'react-bootstrap/Nav';
import NavDropdown from 'react-bootstrap/NavDropdown';
import './App.css'
import Spinner from 'react-bootstrap/Spinner';
import Toast from 'react-bootstrap/Toast';
import ToastContainer from 'react-bootstrap/ToastContainer';
import { background } from '@chakra-ui/react';
import { Alert } from 'react-bootstrap';


/*
需要的数据
功能1：排行榜
  用户名
  用户id
  总分
  一场比赛中每一道题的得分

功能2：提交单个程序
  把提交源码上传到后端

功能3：渲染评测状态和每个数据点结果
评测状态
  题目id
  用户id
  题目名称
  语言
  提交状态
  分数
  提交时间

数据点结果
  测试点序号（第0点表示编译）
  状态
  每一个测试点的时间
  空间
  分数
  评测信息
*/

const MainPage = () => {
  return (
    <Router>
      <Nav fill variant="tabs" defaultActiveKey="/leaderboard">
        <Nav.Item>
          <Nav.Link as={NavLink} to="/home" active>
            主界面
          </Nav.Link>
        </Nav.Item>
        <Nav.Item>
          <Nav.Link as={NavLink} to="/leaderboard" active>
            排行榜
          </Nav.Link>
        </Nav.Item>
        <Nav.Item>
          <Nav.Link as={NavLink} to="/evaluation" active>
            测评信息
          </Nav.Link>
        </Nav.Item>
        <Nav.Item>
          <Nav.Link as={NavLink} to="/submit" active>
            提交代码
          </Nav.Link>
        </Nav.Item>
      </Nav>

      <div className="main-content">
        <Routes>
          <Route path="/home" element={<HomePage />} />
          <Route path="/leaderboard" element={<Leaderboard />} />
          <Route path="/evaluation" element={<EvaluationInfo />} />
          <Route path="/submit" element={<SubmitCode/>} />
        </Routes>
      </div>
    </Router>
  );
};

const HomePage = () => {
  return (
    <div className="home-page">
      <>
      <Spinner animation="border" size="sm" />
      <Spinner animation="border" />
      <Spinner animation="grow" size="sm" />
      <Spinner animation="grow" />
      <Spinner animation="border" variant="primary" />
      <Spinner animation="border" variant="secondary" />
      <Spinner animation="border" variant="success" />
      <Spinner animation="border" variant="danger" />
      <Spinner animation="border" variant="warning" />
      <Spinner animation="border" variant="info" />
      <Spinner animation="border" variant="light" />
      <Spinner animation="border" variant="dark" />
      <Spinner animation="grow" variant="primary" />
      <Spinner animation="grow" variant="secondary" />
      <Spinner animation="grow" variant="success" />
      <Spinner animation="grow" variant="danger" />
      <Spinner animation="grow" variant="warning" />
      <Spinner animation="grow" variant="info" />
      <Spinner animation="grow" variant="light" />
      <Spinner animation="grow" variant="dark" />
    </>
      <h1>Hi! Welcome to a great online judge system!</h1>


    <ToastContainer className="p-3">      
      <Toast style={{ background: 'rgb(0, 201, 87)' }}>
        <Toast.Header closeButton={false}>
          <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
          <strong className="me-auto">C</strong>
          <small>1972</small>
        </Toast.Header>
        <Toast.Body style={{ fontWeight: 'bold' , color: 'white'}}>printf("Hello, World");</Toast.Body>      
        </Toast>
    </ToastContainer>

    <ToastContainer className="p-3" style={{ transform: 'translate(150px, 350px)' }}>
      <Toast style={{ background: 'pink' }}>
        <Toast.Header closeButton={false}>
          <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
          <strong className="me-auto">C++</strong>
          <small>1980</small>
        </Toast.Header>
        <Toast.Body style={{ fontWeight: 'bold' }}>std::cout{"<"}{"<"}"Hello World";</Toast.Body>      
      </Toast>
    </ToastContainer>

    <ToastContainer className="p-3" style={{ transform: 'translate(100px, 200px)' }}>
      <Toast style={{ background: 'blue' }}>
        <Toast.Header closeButton={false}>
          <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
          <strong className="me-auto">C#</strong>
          <small>2001</small>
        </Toast.Header>
        <Toast.Body style={{ fontWeight: 'bold', color: 'white' }}>Console.WriteLine("Hello, World");</Toast.Body>      
      </Toast>
    </ToastContainer>

    <Toast className="custom-toast-container" style={{ transform: 'translate(-650px, 230px)' , background : 'gray'}}>
      <Toast.Header closeButton={false}>
        <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
        <strong className="me-auto">Bash</strong>
        <small>1988</small>
      </Toast.Header>
      <Toast.Body style={{ fontWeight: 'bold', color: 'white' }}>echo "Hello, World"</Toast.Body>      
    </Toast>

    <ToastContainer className="p-3" position='middle-center'>
      <Toast style={{ background: 'rgb(135, 206, 250)' }}>
        <Toast.Header closeButton={false}>
          <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
          <strong className="me-auto">Basic</strong>
          <small>1964</small>
        </Toast.Header>
        <Toast.Body style={{ fontWeight: 'bold' }}>PRINT "Hello, World"</Toast.Body>      
      </Toast>
    </ToastContainer>

    <Toast className="custom-toast-container" style={{ transform: 'translate(-600px, -90px)', background : 'orange'}}>
      <Toast.Header closeButton={false}>
        <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
        <strong className="me-auto">HTML</strong>
        <small>1990</small>
      </Toast.Header>
      <Toast.Body style={{ fontWeight: 'bold' }}> Hello, World</Toast.Body>      
    </Toast>

    <ToastContainer className="p-3" style={{ transform: 'translate(600px, 100px)' }}>
      <Toast style={{ backgroundColor: 'black' }}>
        <Toast.Header closeButton={false}>
          <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
          <strong className="me-auto">Java</strong>
          <small>1995</small>
        </Toast.Header>
        <Toast.Body style={{ fontWeight: 'bold', color: 'white' }}>System.out.println("Hello, World!");</Toast.Body>      
      </Toast>
    </ToastContainer>
    
    <ToastContainer  position='top-end' style={{ marginTop: '90px' }}>
      <Toast style={{ background: 'red' }}>
        <Toast.Header closeButton={false}>
          <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
          <strong className="me-auto">Rust</strong>
          <small>2006</small>
        </Toast.Header>
        <Toast.Body style={{ fontWeight: 'bold', color: 'white'}}>println!("Hello, World")</Toast.Body>      
      </Toast>
    </ToastContainer>

    <Toast className="custom-toast-container" style={{ transform: 'translate(270px, -70px)', background: 'rgb(152, 251, 152)' }}>
      <Toast.Header closeButton={false}>
        <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
        <strong className="me-auto">Delphi</strong>
        <small>1995</small>
      </Toast.Header>
      <Toast.Body style={{ fontWeight: 'bold' }}>printf("Hello, World");</Toast.Body>      
    </Toast>

    <Toast className="custom-toast-container" style={{ transform: 'translate(50px, 70px)', background: 'rgb(255, 215, 0)' }}>
      <Toast.Header closeButton={false}>
        <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
        <strong className="me-auto">MatLab</strong>
        <small>1984</small>
      </Toast.Header>
      <Toast.Body style={{ fontWeight: 'bold' }}>disp("Hello, World");</Toast.Body>      
    </Toast>

    <Toast className="custom-toast-container" style={{ transform: 'translate(-180px, 200px)', background: 'rgb(255, 127, 80)' }}>
      <Toast.Header closeButton={false}>
        <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
        <strong className="me-auto">JavaScript</strong>
        <small>1995</small>
      </Toast.Header>
      <Toast.Body style={{ fontWeight: 'bold' }}>document.write('Hello, World');</Toast.Body>      
    </Toast>

    <Toast className="custom-toast-container" style={{ transform: 'translate(150px, -200px)', background: 'rgb(245, 222, 179)' }}>
      <Toast.Header closeButton={false}>
        <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
        <strong className="me-auto">Pascal</strong>
        <small>1970</small>
      </Toast.Header>
      <Toast.Body style={{ fontWeight: 'bold' }}>WriteLn('Hello, World');</Toast.Body>      
    </Toast>

    <Toast className="custom-toast-container" style={{ transform: 'translate(-300px, -200px)' , background: 'rgb(148, 0, 211)' }}>
      <Toast.Header closeButton={false}>
        <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
        <strong className="me-auto">Objective-C</strong>
        <small>1984</small>
      </Toast.Header>
      <Toast.Body style={{ fontWeight: 'bold', color: 'white' }}>NSLog(@"Hello, World");</Toast.Body>      
    </Toast>

    <Toast style={{ transform: 'translate(1000px, 400px)', background: 'rgb(0, 75, 155)'}}>
      <Toast.Header closeButton={false}>
        <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
        <strong className="me-auto">R</strong>
        <small>1980</small>
      </Toast.Header>
      <Toast.Body style={{ fontWeight: 'bold', color: 'white' }}>cat("Hello, World")</Toast.Body>      
    </Toast>

    <Toast style={{ transform: 'translate(1000px, 150px)', background: 'rgb(183, 96, 96)' }}>
      <Toast.Header closeButton={false}>
        <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
        <strong className="me-auto">Ruby</strong>
        <small>1993</small>
      </Toast.Header>
      <Toast.Body style={{ fontWeight: 'bold', color: 'white' }}>puts "Hello, World"</Toast.Body>      
    </Toast>

    <Toast style={{ transform: 'translate(620px, 340px)' , background: 'rgb(244, 206, 176)' }}>
      <Toast.Header closeButton={false}>
        <img src="holder.js/20x20?text=%20" className="rounded me-2" alt=""/>
        <strong className="me-auto">Python</strong>
        <small>1991</small>
      </Toast.Header>
      <Toast.Body style={{ fontWeight: 'bold' }}>print ("Hello, World")	</Toast.Body>      
    </Toast>

    </div>
  );
};

const Leaderboard = () => {
  const [leaderboardData, setLeaderboardData] = useState<{ rank: number; username: string; userid: number; scores: number[] }[]>([]);
  const [invalidArgumentError, setInvalidArgumentError] = useState<string | null>(null);
  const [invalidStateError, setInvalidStateError] = useState<string | null>(null);
  const [notFoundError, setNotFoundError] = useState<boolean>(false);
  const [rateLimitError, setRateLimitError] = useState<boolean>(false);
  const [externalError, setExternalError] = useState<boolean>(false);
  const [internalError, setInternalError] = useState<boolean>(false);
  const [show, setShow] = useState(true);

  useEffect(() => {
    const fetchRankList = async (contestId: number, queryParam: string) => {
      try {
        // 清空leaderboardData中的数据
        setLeaderboardData([]);
        clearErrors();

        const apiUrl = `http://localhost:12345/contests/${contestId}/ranklist${queryParam}`;
        const response = await axios.get(apiUrl);
        const rankData = response.data as { 
          user: { id: number; name: string };
          rank: number;
          scores: number[];
        }[];
        const updatedLeaderboardData = rankData.map((rankEntry) => ({
          rank: rankEntry.rank,
          username: rankEntry.user.name,
          userid: rankEntry.user.id,
          scores: rankEntry.scores,
        }));

        setLeaderboardData(updatedLeaderboardData);
      } catch (error : any) {
        console.log(error);
        if (error.response && error.response.data) {
          const { reason, code } = error.response.data;
          if (code === 1) {
            setInvalidArgumentError(`Invalid argument error: ${reason}`);
          } else if (code === 2) {
            setInvalidStateError(`Invalid state error: ${reason}`);
          } else if (code === 3) {
            setNotFoundError(true);
          } else if (code === 4) {
            setRateLimitError(true);
          } else {
            setInternalError(true);
          }
        } else {
          setExternalError(true);
        }
      }
    };
    // 从前端获取contestId和queryParam
    const contestId = 0; // 替换为实际的比赛ID
    const queryParam = '?scoring_rule=highest'; // 替换为实际的查询参数
    fetchRankList(contestId, queryParam);
  }, []); //configData改变时又改变
  const clearErrors = () => {
    setInvalidArgumentError(null);
    setInvalidStateError(null);
    setNotFoundError(false);
    setRateLimitError(false);
    setExternalError(false);
    setInternalError(false);
  };
  return (
    <div>
      {invalidArgumentError && 
      <div>
        <Alert variant="danger" onClose={() => setShow(false)} dismissible>
          <Alert.Heading>Oh snap! Error: {invalidArgumentError}</Alert.Heading>
          <p>
            You may want to consider the following approaches to address this problem:          
          </p>
          <p>
            1. Double-check the accuracy of the parameters provided in the request and ensure they align with the API documentation.
          </p>
          <p>
            2. Verify that the parameters meet the required format, type, and value range.
          </p>
          <p>
            3. Ensure that no mandatory parameters have been overlooked or omitted.
          </p>
          <p>
            4. Validate the proper encoding of parameters, especially when dealing with special characters or spaces.
          </p>
        </Alert>
      </div>
      }
      {invalidStateError &&
      <div>
        <Alert variant="danger" onClose={() => setShow(false)} dismissible>
          <Alert.Heading>Oh snap! Error: {invalidStateError}</Alert.Heading>
          <p>
            You may want to consider the following approaches to address this problem:
          </p>
          <p>
            1. Ensure that the object's state matches the required state for the operation.
          </p>
          <p>
            2. Check if any prior actions have altered the object's state, making the current operation inappropriate.
          </p>
          <p>
            3. Follow the correct sequence of steps, ensuring that all prerequisites are fulfilled before performing the specific operation.
          </p>
          <p>
            4. If feasible, restore the object to a valid state and retry the operation.
          </p>
        </Alert>
      </div>
      }
      {notFoundError && 
      <div>
      <Alert variant="danger" onClose={() => setShow(false)} dismissible>
        <Alert.Heading>Oh snap! You got an NOT FOUND error!</Alert.Heading>
          <p>
            You may want to consider the following approaches to address this problem:
          </p>
          <p>
            1. Validate if the requested object exists within the system.
          </p>
          <p>
            2. Verify that the object's identifier or key attributes are correct and match the corresponding object in the system.
          </p>
          <p>
            3. Ensure that the access permissions and scope for the object are properly configured.
          </p>
          <p>
            4. If the object has been deleted or moved, update the requested object's location accordingly, as needed.
          </p>
        </Alert>
      </div>
      }
      {rateLimitError && 
        <div>
          <Alert variant="danger" onClose={() => setShow(false)} dismissible>
            <Alert.Heading>Oh snap! Your submission has exceeded the limit!</Alert.Heading>
            <p>
              Sorry, but you are unable to continue submitting evaluations.
            </p>
            <p>
              Continue to work on the next question!
            </p>
          </Alert>
        </div>
      }
      {externalError && 
      <div>
        <Alert variant="danger" onClose={() => setShow(false)} dismissible>
          <Alert.Heading>Oh snap! You got an EXTERNAL error!</Alert.Heading>
          <p>
          You may want to consider the following approaches to address this problem:
          </p>
          <p>
            1. Make sure to verify the connectivity with the external system, such as the database, is functioning as intended.
          </p>
          <p>
            2. Double-check the network connection and firewall configurations to ensure seamless communication with the external system.
          </p>
          <p>
            3. Ensure that the external system's configuration and access credentials are accurate and up to date.
          </p>
          <p>
            4. Keep a close eye on the operational status of the external system and take necessary remedial actions, such as restarting or troubleshooting the external system if needed.
          </p>
        </Alert>
      </div>
      }
      {internalError && 
      <div>
        <Alert variant="danger" onClose={() => setShow(false)} dismissible>
          <Alert.Heading>Oh snap! You got an INTERNAL error!</Alert.Heading>
          <p>
            You may want to consider the following approaches to address this problem:
          </p>
          <p>
            1. Review server logs to gather additional details about the internal exception.
          </p>
          <p>
            2. Verify the correctness of server-side code and logic, eliminating potential programming errors.
          </p>
          <p>
            3. Inspect server configuration and environment to ensure smooth operation.
          </p>
          <p>
            4. If feasible, restart the server and monitor its behavior to promptly identify and address any underlying issues.
          </p>
        </Alert>
      </div>
      }
      
      <Table striped bordered hover>
      <thead>
        <tr>
          <th>#</th>
          <th>User ID</th>
          <th>User Name</th>
          <th>Total Score</th>
        </tr>
      </thead>
      <tbody>
        {leaderboardData.map((entry, index) => (
          <tr key={index}>
            <td>{entry.rank}</td>
            <td>{entry.userid}</td>
            <td>{entry.username}</td>
            <td>
              {entry.scores.reduce((sum, score) => sum + score, 0)}
            </td>
            
            {entry.scores.map((score, scoreIndex) => (
              <td>
                <span key={scoreIndex}>{score} </span>
              </td>
            ))}
            
          </tr>
        ))}
      </tbody>
    </Table>
    </div>
  );
};

const EvaluationInfo = () => {

  function getVariantByState(state: String): string {
    if (state === 'Waiting') {
      return "secondary";
    } else if (state === 'Compilation Success' || state === 'Running') {
      return "info";
    } else if (state === 'Accepted' ) {
      return "success";
    } else if (state === 'Runtime Error' || state === 'Memory Limit Exceeded' || state === 'Time Limit Exceeded') {
      return "warning";
    } else if (state === 'System Error' || state === 'Compilation Error') {
      return "primary";
    } else if (state === 'SPJ Error' || state === 'Wrong Answer') {
      return "danger";
    } else if (state === 'Skipped' ) {
      return "dark";
    } else {
      return "default";
    }
  }

  const [jobcasesData, setjobcasesData] = useState<{ result: String, timeconsume: Number, memory: Number, info: String}[]>([]);
  const [jobresultData, setjobresultData] = useState<{ problemid: Number, userid: Number, language: String, result: String, score: Number, createdtime: String}[]>([]);  
  const [invalidArgumentError, setInvalidArgumentError] = useState<string | null>(null);
  const [invalidStateError, setInvalidStateError] = useState<string | null>(null);
  const [notFoundError, setNotFoundError] = useState<boolean>(false);
  const [rateLimitError, setRateLimitError] = useState<boolean>(false);
  const [externalError, setExternalError] = useState<boolean>(false);
  const [internalError, setInternalError] = useState<boolean>(false);
  const [show, setShow] = useState(true);


  useEffect(() => {

    const fetchcasesData = async () => {
      try {
        setjobcasesData([]);
        setjobresultData([]);
        console.log(jobresultData);
        const response = await axios.get('http://localhost:12345/jobs');
        const jobcases = response.data[response.data.length - 1]  as { 
          id: Number,
          created_time: String,
          updated_time: String,
          submission: {source_code: String, language: String, user_id: Number, contest_id: Number, problem_id: Number}
          state: String,
          result: string,
          score: number;
          cases: {id: Number, result: String, time: Number, memory: Number, info: String}[];
        };

        const updatedjobcasesData = jobcases.cases.map((caseEntry) => ({
          result: caseEntry.result,
          timeconsume: caseEntry.time,
          memory: caseEntry.memory,
          info: caseEntry.info,
        }));
        
        const updatedresultData = {
          problemid: jobcases.submission.problem_id,
          userid: jobcases.submission.user_id,
          language: jobcases.submission.language,
          result: jobcases.result,
          score: jobcases.score,
          createdtime: jobcases.created_time,
        };

        setjobcasesData(updatedjobcasesData);
        console.log(updatedjobcasesData);

        setjobresultData([updatedresultData]);
      } catch (error : any) {
        console.log(error);
        if (error.response && error.response.data) {
          const { reason, code } = error.response.data;
          console.log(code);

          if (code === 1) {
            setInvalidArgumentError(`Invalid argument error: ${reason}`);
          } else if (code === 2) {
            setInvalidStateError(`Invalid state error: ${reason}`);
          } else if (code === 3) {
            setNotFoundError(true);
          } else if (code === 4) {
            setRateLimitError(true);
          } else {
            setInternalError(true);
          }
        } else {
          setExternalError(true);
        }
      }
    };
    fetchcasesData();
  }, []);

  return (
    <div>
      {invalidArgumentError && 
      <div>
        <Alert variant="danger" onClose={() => setShow(false)} dismissible>
          <Alert.Heading>Oh snap! Error: {invalidArgumentError}</Alert.Heading>
          <p>
            You may want to consider the following approaches to address this problem:          
          </p>
          <p>
            1. Double-check the accuracy of the parameters provided in the request and ensure they align with the API documentation.
          </p>
          <p>
            2. Verify that the parameters meet the required format, type, and value range.
          </p>
          <p>
            3. Ensure that no mandatory parameters have been overlooked or omitted.
          </p>
          <p>
            4. Validate the proper encoding of parameters, especially when dealing with special characters or spaces.
          </p>
        </Alert>
      </div>
      }
      {invalidStateError &&
      <div>
        <Alert variant="danger" onClose={() => setShow(false)} dismissible>
          <Alert.Heading>Oh snap! Error: {invalidStateError}</Alert.Heading>
          <p>
            You may want to consider the following approaches to address this problem:
          </p>
          <p>
            1. Ensure that the object's state matches the required state for the operation.
          </p>
          <p>
            2. Check if any prior actions have altered the object's state, making the current operation inappropriate.
          </p>
          <p>
            3. Follow the correct sequence of steps, ensuring that all prerequisites are fulfilled before performing the specific operation.
          </p>
          <p>
            4. If feasible, restore the object to a valid state and retry the operation.
          </p>
        </Alert>
      </div>
      }
      {notFoundError && 
      <div>
      <Alert variant="danger" onClose={() => setShow(false)} dismissible>
        <Alert.Heading>Oh snap! You got an NOT FOUND error!</Alert.Heading>
          <p>
            You may want to consider the following approaches to address this problem:
          </p>
          <p>
            1. Validate if the requested object exists within the system.
          </p>
          <p>
            2. Verify that the object's identifier or key attributes are correct and match the corresponding object in the system.
          </p>
          <p>
            3. Ensure that the access permissions and scope for the object are properly configured.
          </p>
          <p>
            4. If the object has been deleted or moved, update the requested object's location accordingly, as needed.
          </p>
        </Alert>
      </div>
      }
      {rateLimitError && 
        <div>
          <Alert variant="danger" onClose={() => setShow(false)} dismissible>
            <Alert.Heading>Oh snap! Your submission has exceeded the limit!</Alert.Heading>
            <p>
              Sorry, but you are unable to continue submitting evaluations.
            </p>
            <p>
              Continue to work on the next question!
            </p>
          </Alert>
        </div>
      }
      {externalError && 
      <div>
        <Alert variant="danger" onClose={() => setShow(false)} dismissible>
          <Alert.Heading>Oh snap! You got an EXTERNAL error!</Alert.Heading>
          <p>
          You may want to consider the following approaches to address this problem:
          </p>
          <p>
            1. Make sure to verify the connectivity with the external system, such as the database, is functioning as intended.
          </p>
          <p>
            2. Double-check the network connection and firewall configurations to ensure seamless communication with the external system.
          </p>
          <p>
            3. Ensure that the external system's configuration and access credentials are accurate and up to date.
          </p>
          <p>
            4. Keep a close eye on the operational status of the external system and take necessary remedial actions, such as restarting or troubleshooting the external system if needed.
          </p>
        </Alert>
      </div>
      }
      {internalError && 
      <div>
        <Alert variant="danger" onClose={() => setShow(false)} dismissible>
          <Alert.Heading>Oh snap! You got an INTERNAL error!</Alert.Heading>
          <p>
            You may want to consider the following approaches to address this problem:
          </p>
          <p>
            1. Review server logs to gather additional details about the internal exception.
          </p>
          <p>
            2. Verify the correctness of server-side code and logic, eliminating potential programming errors.
          </p>
          <p>
            3. Inspect server configuration and environment to ensure smooth operation.
          </p>
          <p>
            4. If feasible, restart the server and monitor its behavior to promptly identify and address any underlying issues.
          </p>
        </Alert>
      </div>
      }
      <Table striped bordered hover>
      <thead>
        <tr>
          <th>#</th>
          <th>题目</th>
          <th>语言</th>
          <th>状态</th>
          <th>分数</th>
          <th>时间</th>
        </tr>
      </thead>
        <tbody>
         {jobresultData.map((entry, index) => (
             <tr key={index}>
                <td>{entry.userid.toString()}</td>
                <td>{entry.problemid.toString()}</td>
                <td>{entry.language}</td>
                <td>
                  <ListGroup.Item  variant={getVariantByState(entry.result)}>
                    {entry.result}
                  </ListGroup.Item>
                </td>
                
                <td>{entry.score.toString()}</td>
                <td>{entry.createdtime}</td>             
             </tr>
           ))}
         </tbody>
    </Table>

    <Table striped bordered hover>
      <thead>
        <tr>
          <th>#</th>
          <th>状态</th>          
          <th>时间</th>
          <th>空间</th>
          <th>评测信息</th>
        </tr>
      </thead>
        <tbody>
         {jobcasesData.map((entry, index) => (
             <tr key={index}>
                <td>{index}</td>
                <td>
                  <ListGroup.Item  variant={getVariantByState(entry.result)}>
                    {entry.result}
                  </ListGroup.Item>
                </td>
                <td>{entry.timeconsume.toString()}</td>
                <td>{entry.memory.toString()}</td>
                <td>{entry.info}</td>
             </tr>
           ))}
         </tbody>
    </Table>
    </div>
  );
};

const SubmitCode = () => {
  const [fileContent, setFileContent] = useState('');
  const [selectedOption, setSelectedOption] = useState('');
  const [problemNumber, setProblemNumber] = useState('');
  const [contestNumber, setContestNumber] = useState('');
  const [userID, setUserID] = useState('');
  const [configData, setConfigData] = useState<{ languages: string[]; problems: number[]}>();

  useEffect(() => {
    const fetchConfigData = async () => {
      try {
        const response = await axios.get('http://localhost:12345/config'); // 根据实际的后端路由配置来发送请求
        const data = response.data as {
          server:{bind_address: string; bind_port: number};
          problems: {
            id: number; 
            name: string; 
            type: string, 
            misc: {packing: any; special_judge: any; dynamic_ranking_ratio: any};
            cases: {score: number; input_file: string; answer_file: string; time_limit: number; memory_limit: number}[]
          }[];
          languages: {
            name: string; 
            file_name: string;
            command: string[];
          }[]
        };
        const configData = {
          languages: data.languages.map((language) => language.name),
          problems: data.problems.map((problem) => problem.id),
        };

        setConfigData(configData);
        console.log(configData);
      } catch (error) {
        console.log(error);
      }
    };

    fetchConfigData();
  }, []);

  const handleSubmit = async () => {
    setFileContent('');
    try {
      console.log("ready to submit");
      // 构建要提交的数据对象
      const data = {
        // 构建要提交的数据对象
        source_code: fileContent,
        language: selectedOption,
        user_id: parseInt(userID),
        contest_id: parseInt(contestNumber),
        problem_id: parseInt(problemNumber),
      };
      console.log(fileContent);
      console.log(data);
      // 发送 POST 请求到后端
      const response = await axios.post('http://localhost:12345/jobs', data);
      // 处理后端的响应
      console.log(response.data);
    } catch (error) {
      console.error(error);
    }
  };
  console.log(configData?.languages);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const fileList = event.target.files;
    if (fileList && fileList.length > 0) {
      const file = fileList[0];
      const reader = new FileReader();
      reader.onload = (e) => {
        const content = e.target?.result?.toString() || '';
        setFileContent(content);
      };
      reader.readAsText(file);
    }
  };
  
  const handleReset = () => {
    setFileContent('');
  };

  const handleOptionChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
    // console.log(event.target.value);
    setSelectedOption(event.target.value);
  };

    return (
    <>
      {/*递交答案*/}
      <Stack direction='vertical' gap={1}>
        {/*选择文件 文件选择成功后将写入文本框*/}
        <Form.Label style={{ fontWeight: 'bold', fontSize: '3 rem'}}>递交答案</Form.Label>

        <div>
          <Stack direction='horizontal' gap={(3)}>
            <Form.Control type="text" value={userID} onChange={(e) => setUserID(e.target.value)} placeholder="User ID"/>
            <Form.Select aria-label="Default select example" value={problemNumber} onChange={(e) => setProblemNumber(e.target.value)}>
              <option>Problem ID</option>
              {configData && configData.problems.map((problem) => (
                <option key={problem} value={problem}>{problem}</option>
              ))}
            </Form.Select>
            <Form.Control type="text" value={contestNumber} onChange={(e) => setContestNumber(e.target.value)} placeholder="Contest ID"/>
          </Stack>
          <Form.Select aria-label="Default select example" value={selectedOption} onChange={handleOptionChange}>
          <option>语言和编译选项</option>
          {configData && configData.languages.map((language) => (
            <option key={language} value={language}>{language}</option>
          ))}
        </Form.Select>
        </div>
        <Form.Control type="file" onChange={handleFileChange}/>
        {/*文本框*/}
        <Form.Control as="textarea" rows={10} value={fileContent} onChange={(e) => setFileContent(e.target.value)}/>
        {/*提交和清空*/}
        <Stack direction='horizontal' gap={3} className="d-flex justify-content-end align-items-center">
          <Button variant="success" size="lg" onClick={handleSubmit}>Submit</Button>
          <Button variant="outline-danger" size="lg" onClick={handleReset}>Reset</Button>
        </Stack>
      </Stack> 
    </>
  );
}

function allFormFile() {
  //     <Form.Label htmlFor="inputPassword5">Password</Form.Label>
  //       <InputGroup className="mb-3">
  //         <Form.Control
  //           type="password"
  //           id="inputPassword5"
  //           aria-describedby ="passwordHelpBlock"
  //         />
  //         <Button variant="outline-secondary" id="button-addon1">
  //           Button
  //         </Button>
  //         <Form.Text id="passwordHelpBlock" muted>
  //         Your password must be 8-20 characters long, contain letters and numbers,
  //         and must not contain spaces, special characters, or emoji.
  //       </Form.Text>
  //     </InputGroup>


  //     <InputGroup className="mb-3">
  //       <Form.Control
  //         placeholder="Recipient's username"
  //         aria-label="Recipient's username"
  //         aria-describedby="basic-addon2"
  //       />
  //       <Button variant="outline-secondary" id="button-addon1">
  //         Button
  //       </Button>
  //     </InputGroup>

  //     <Form.Floating className="mb-3">
  //       <Form.Control
  //         id="floatingInputCustom"
  //         type="email"
  //         placeholder="name@example.com"
  //       />
  //       <label htmlFor="floatingInputCustom">Email address</label>
  //     </Form.Floating>
  //     <Form.Floating>
  //       <Form.Control
  //         id="floatingPasswordCustom"
  //         type="password"
  //         placeholder="Password"
  //       />
  //       <label htmlFor="floatingPasswordCustom">Password</label>
  //     </Form.Floating>

  //     <Alert variant="success">
  //     <Alert.Heading>Hey, nice to see you</Alert.Heading>
  //     <p>
  //       Aww yeah, you successfully read this important alert message. This
  //       example text is going to run a bit longer so that you can see how
  //       spacing within an alert works with this kind of content.
  //     </p>
  //     <hr />
  //     <p className="mb-0">
  //       Whenever you need to, be sure to use margin utilities to keep things
  //       nice and tidy.
  //     </p>
  //   </Alert>

  //   <ListGroup>
  //       <ListGroup.Item action className="small-list-item">
  //         default
  //       </ListGroup.Item>
  //       <ListGroup.Item action variant="primary">
  //         Compalition Error & System Error
  //       </ListGroup.Item>
  //       <ListGroup.Item action variant="secondary">   	
  //         Waiting
  //       </ListGroup.Item>
  //       <ListGroup.Item action variant="success">
  //         Accepted
  //       </ListGroup.Item>
  //       <ListGroup.Item action variant="danger">
  //         Wrong Answer & SPJ Error
  //       </ListGroup.Item>
  //       <ListGroup.Item action variant="warning">
  //         Time Limit Exceeded & Runtime Error & Memory Limit Exceeded
  //       </ListGroup.Item>
  //       <ListGroup.Item action variant="info">
  //         Compalition Success
  //       </ListGroup.Item>
  //       <ListGroup.Item action variant="light">
  //         vacant
  //       </ListGroup.Item>
  //       <ListGroup.Item action variant="dark">
  //         Skipped
  //       </ListGroup.Item>
  //     </ListGroup>
  //   </>
  // );
  return (
    <MainPage />
  );
}

export default allFormFile;